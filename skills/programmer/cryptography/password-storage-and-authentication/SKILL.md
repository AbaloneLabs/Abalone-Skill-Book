---
name: password_storage_and_authentication.md
description: Use when the agent is storing, hashing, verifying, or migrating user passwords; choosing a password hashing function (bcrypt, scrypt, Argon2id, PBKDF2) and tuning its cost parameters; designing a login or password-check flow; implementing rate limiting or lockout against credential guessing; deciding password policy (length, complexity, rotation); handling password resets and recovery; defending against account enumeration via login, reset, or registration responses; migrating hashes when algorithms or cost factors change; or reviewing a password system for offline-attack resistance. Also covers the distinction between online and offline attack surfaces, why fast hashes fail for passwords, pepper usage, and breach resilience.
---

# Password Storage And Authentication

Password storage is judged not by how it performs when everything is fine, but by how it resists the day the password database is stolen. That day is the entire threat model: a slow, salted, memory-hard hash turns a wholesale breach into an expensive guessing exercise, while a fast hash or reversible encryption turns it into instant mass credential recovery. The recurring failure is not a missing feature; it is choosing a mechanism whose properties are right for the happy path (fast verification) and catastrophic for the breach path (fast cracking). A login that works perfectly in production can be a total credential compromise the moment the database file leaves the building.

Agents tend to under-invest here because password verification "just works" with a fast hash, and the breach scenario feels hypothetical during feature work. The defects are invisible until the worst day: a SHA-256 hash cracks billions of guesses per second on a GPU; a missing salt means one rainbow table breaks every reused password; a low cost factor that was fine a decade ago is now trivial; a login error that says "user not found" versus "wrong password" lets an attacker enumerate the user base; a password policy optimized for complexity produces passwords humans write on sticky notes. The judgment problem is designing the password system for the adversary who already has the database, not only for the user who is logging in.

This skill is about breach-resilient password storage and the authentication flows around it. It complements the encryption skill (which covers the hashing-versus-encryption distinction and key management) and the session skill (which covers what happens after authentication succeeds). Here the question is how to store and verify passwords so that a database theft does not become a mass account takeover.

## Core Rules

### Store Passwords With A Slow, Salted, Memory-Hard Hash — Nothing Else

Password storage has one correct answer and many wrong ones that compile and verify correctly. The correct answer is a purpose-built password hashing function with a per-password random salt and parameters tuned to be deliberately slow.

- **Use Argon2id (preferred), bcrypt, or scrypt.** These are designed to be expensive — in CPU, and for Argon2id/scrypt, in memory — so that guessing is slow and GPU/ASIC acceleration is blunted. PBKDF2 is acceptable where the others are unavailable, with a high iteration count.
- **Never store plaintext, reversibly encrypted, or fast-hashed passwords.** Fast hashes (MD5, SHA-1, SHA-256, SHA-512) are designed for speed; an attacker with the hash file tries billions of guesses per second on commodity GPUs. Reversible encryption means a key compromise recovers every password.
- **Always use a per-password random salt.** The salt prevents precomputation (rainbow tables) and ensures identical passwords produce different hashes, so an attacker must crack each hash individually. The salt is not secret; it is stored alongside the hash.
- **Encode the algorithm, version, and parameters in the stored hash** (the modular crypt format or a structured record), so future migration and parameter changes are unambiguous.

The decisive test: if an attacker obtains the entire password table and the application source, how long does one guess take? With a fast hash, milliseconds; with a tuned Argon2id, a deliberate fraction of a second. That ratio is the entire defense.

### Tune The Cost Factor For The Hardware And The Threat

A password hash is only as strong as its cost. The same algorithm at a low cost factor is a fast hash in disguise. Cost must be tuned deliberately and revisited as hardware improves.

- **Set the cost so a single verification takes a target wall-clock time** (commonly tens to low hundreds of milliseconds) on your production hardware. Faster than that, and offline cracking is too easy; slower than that, and login throughput and denial-of-service resistance suffer.
- **Memory-hard parameters (Argon2id) should use a meaningful amount of memory** (tens to hundreds of MB), so an attacker cannot parallelize many guesses on a GPU with limited memory per core.
- **Re-tune periodically.** Hardware gets faster; a cost factor tuned in 2015 is weak in 2025. Treat the cost factor as a configuration that is reviewed and raised, not set once.
- **Beware denial of service.** A very high cost protects against cracking but makes a login endpoint a cheap DoS target (an attacker floods login attempts, each burning CPU). Pair a high hash cost with login rate limiting.

The cost factor is the single most important parameter after the algorithm choice, and it is the one most often left at a stale default. Document the target verification time and the date it was last tuned.

### Defend The Online Surface Separately From The Offline Surface

Password security has two attack surfaces with different defenses, and confusing them leaves one undefended.

- **Offline attack** (attacker has the hash table) is defended by the slow, salted hash. No rate limit helps here; the attacker cracks offline at full speed.
- **Online attack** (attacker guesses against the live login endpoint) is defended by rate limiting, lockout/throttling, and increasingly by multi-factor authentication. The hash strength is irrelevant if the endpoint just tells the attacker when a guess is right.

Defend both. A strong hash with an unprotected login endpoint falls to online guessing; a rate-limited endpoint with a fast hash falls the day the database leaks. The two defenses are complementary, not substitutes.

For the online surface, prefer progressive throttling or exponential backoff per account and per source over hard lockouts, because hard lockouts create a denial-of-service vector (an attacker locks out accounts by guessing wrong on purpose). Consider CAPTCHA or stepped-up verification only after sustained failures, and never reveal whether the failure was "user not found" versus "wrong password."

### Prevent Account Enumeration In Every Auth-Adjacent Flow

Account enumeration is the leak of whether a username or email exists in the system. It is a precondition for credential stuffing, targeted phishing, and privacy harm, and it leaks through many flows beyond login.

- **Login and registration.** Return the same generic error ("invalid credentials") whether the account does not exist or the password is wrong. Do not reveal "user not found." On registration, avoid "this email is already registered" where possible, or accept the tradeoff deliberately (some flows need it for UX).
- **Password reset.** Show the same confirmation ("if an account exists for this address, a reset link has been sent") regardless of whether the account exists. Sending a reset email only when the account exists confirms membership to anyone who can submit the form.
- **Timing.** A login for a non-existent user that returns instantly, versus a real user that takes 200ms to hash the password, leaks membership through timing. Equalize the work: perform a hash verification against a dummy hash even when the account does not exist, so the timing is constant.
- **Responses to password change, MFA enrollment, and account recovery** must likewise avoid distinguishing existing from non-existing accounts.

Enumeration is subtle because each flow feels harmless in isolation. Audit every authentication-adjacent endpoint for whether its response, status code, timing, or side effect (email sent or not) reveals account existence.

### Design Password Resets As Authentication, Not As A Feature

A password reset flow is an alternate authentication path — it grants the ability to set a new password, which is the ability to take over the account. It must be defended with the same rigor as login, and its tokens treated as credentials.

- **Reset tokens must be unguessable, single-use, and short-lived.** A reset link is a bearer credential; generate it with a CSPRNG, bind it to the account, expire it quickly (minutes to low hours), and invalidate it after use.
- **Do not reveal account existence through reset.** See enumeration above.
- **Invalidate existing sessions after a password reset** (or after a password change that follows suspected compromise), because a reset often indicates the user lost control of the account.
- **Notify the user of the change** (email on password change) so an unauthorized reset is noticed, without revealing information that itself enables an attack.
- **Do not email the current or new password in plaintext.** Email is not a secure channel and the inbox may be compromised; send a token-bound link, not a password.

A reset flow that uses a short, predictable token, or that reveals whether the account exists, or that does not invalidate other sessions, is an account-takeover path with a friendlier UI.

### Make Password Policy Serve Humans And Threats, Not A Checklist

Password policy (length, complexity, rotation) is widely misunderstood. The goal is high-entropy passwords that resist guessing, not compliance theater that produces passwords humans hate and reuse.

- **Length is the dominant factor.** Prefer a high minimum length (e.g., 12+ characters) over complex character-class requirements. A long passphrase is stronger and more memorable than a short "P@ssw0rd!" that satisfies complexity and is reused everywhere.
- **Check against breached passwords.** A password that appears in a breach corpus (via a k-anonymity API like HaveIBeenPwned, or a local list) is effectively public; reject it regardless of complexity. This catches the real risk (reuse of known-bad passwords) better than character-class rules.
- **Avoid forced periodic rotation.** Mandatory rotation produces weaker passwords (humans cycle through predictable variants) and offers little against modern threats; modern guidance (NIST 800-63B) recommends rotation only on suspected compromise, not on a schedule.
- **Do not impose arbitrary restrictions** (no special characters, max length of 16) that weaken passwords or break password managers.

The strong policy maximizes effective entropy and rejects known-bad passwords; the weak policy enforces `A-Z a-z 0-9 !@#` and rotates every 30 days.

### Migrate Hashes Without Weakening Or Locking Users Out

When the algorithm or cost factor changes, existing stored hashes must be migrated. The migration must not weaken security during the transition or require a mass password reset.

- **Re-hash on next login.** When a user authenticates successfully against the old hash, immediately compute a new hash with the current algorithm and cost, and replace the stored value. The migration happens transparently over time as users log in.
- **Never downgrade.** A migration must move hashes to a stronger scheme; never rewrite a strong hash as a weaker one.
- **Track the algorithm per hash.** Because migration is gradual, the table will contain a mix of old and new hashes; each record must identify its scheme so verification uses the right one.
- **Plan for stragglers.** Users who never log in again keep the old hash indefinitely; consider a forced re-auth or a sunset for very old schemes, and accept that some accounts remain on the legacy scheme until they return.

A migration that requires all users to reset their passwords is operationally painful and signals a design gap; the re-hash-on-login pattern is the standard, low-friction approach.

## Common Traps

### Fast-Hashing Passwords (SHA-256, MD5) Because The Library Is Convenient

Using a general-purpose hash because it is one function call and verifies in microseconds. An attacker with the hash table tries billions of guesses per second. Use a slow, salted, memory-hard password hashing function.

### Missing Or Shared Salt

Using no salt, or a single salt for all users, so identical passwords produce identical hashes and a rainbow table breaks every reused password at once. Use a per-password random salt stored with the hash.

### Cost Factor Left At A Stale Default

A bcrypt cost of 10 tuned in 2010, still 10 today, when hardware is orders of magnitude faster. Re-tune the cost to the current hardware and a target verification time; treat the cost as a reviewed parameter.

### Revealing "User Not Found" On Login Or Reset

Distinct error messages or response timing that tells an attacker which accounts exist, enabling targeted credential stuffing and phishing. Return generic errors and equalize timing (hash a dummy value for non-existent users).

### Hard Lockout As The Only Online Defense

Locking an account after N failed attempts, which an attacker weaponizes to deny service to legitimate users by intentionally failing their logins. Prefer progressive throttling and per-source rate limits.

### Forced Periodic Rotation Producing Weak Passwords

Mandatory 30-day rotation that leads users to cycle "Spring2024!", "Summer2024!", weakening security. Drop scheduled rotation; rotate only on suspected compromise.

### Reset Token That Is Predictable, Reusable, Or Long-Lived and complexity Rules Without Breached-Password Checking

A reset link based on a timestamp or a short token, valid for days and usable multiple times, emailed in plaintext. Treat reset tokens as short-lived, single-use, unguessable bearer credentials.

Enforcing `1 upper, 1 lower, 1 digit, 1 symbol` while accepting `Password1!` which is in every breach corpus. Check against breached passwords; length and breach-checking beat character-class rules.

### Emailing The Password In Plaintext On Signup Or Reset and migration That Downgrades Or Locks Everyone Out

Sending the user's password (current or new) through email, an insecure channel, where it persists in the inbox. Send a token-bound reset link; never email passwords.

Rewriting hashes to a weaker algorithm during migration, or forcing a mass password reset because the migration was not designed for gradual re-hash-on-login. Migrate by re-hashing on next successful login, never downgrading.

## Self-Check

- [ ] Passwords are stored only with a slow, salted, memory-hard hashing function (Argon2id preferred, bcrypt or scrypt acceptable), never plaintext, reversibly encrypted, or fast-hashed (MD5/SHA-1/SHA-256).
- [ ] Each password has its own random salt stored alongside the hash, and the stored record encodes the algorithm, version, and parameters so verification and migration are unambiguous.
- [ ] The cost factor (iterations, memory, parallelism) is tuned to a target verification time on current production hardware, documented with the date last tuned, and scheduled for periodic re-evaluation as hardware improves.
- [ ] The online attack surface (login endpoint) is defended separately from the offline surface (hash strength) via progressive throttling, per-source rate limiting, and optional stepped-up verification — without hard lockouts that create a denial-of-service vector.
- [ ] No authentication-adjacent flow (login, registration, password reset, MFA enrollment, account recovery) reveals account existence through message text, status code, timing, or side effects; timing is equalized by performing dummy work for non-existent accounts.
- [ ] Password reset tokens are unguessable (CSPRNG), single-use, short-lived, bound to the account, and treated as bearer credentials; existing sessions are invalidated after a reset, the user is notified of the change, and no password is ever emailed in plaintext.
- [ ] Password policy maximizes effective entropy (high minimum length, passphrases encouraged), rejects known-breached passwords via a breach corpus or k-anonymity API, and avoids forced periodic rotation except on suspected compromise.
- [ ] Hash migration uses re-hash-on-next-login to move gradually to a stronger scheme, never downgrades, tracks the algorithm per record, and does not require a mass password reset.
- [ ] The design was evaluated against the breach scenario (attacker has the full hash table and source) and one offline guess takes a deliberate fraction of a second, not milliseconds.
