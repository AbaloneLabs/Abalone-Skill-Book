---
name: php_composer_and_dependency_management.md
description: Use when the agent is writing or reviewing PHP code that uses Composer, managing composer.json and composer.lock, choosing version constraints, resolving dependency conflicts, configuring PSR-4 autoload, deciding require vs require-dev, ensuring deployment reproducibility, handling patches and forks, or migrating a legacy include-based project to Composer-managed dependencies.
---

# Composer And Dependency Management In PHP

Composer is the dependency manager that made modern PHP possible: declared dependencies, semver constraints, a lockfile for reproducibility, and PSR-4 autoloading that replaced the `require_once` sprawl of legacy projects. But Composer's correctness depends on treating `composer.json` and `composer.lock` as load-bearing configuration rather than as files Composer regenerates for you. Loose version constraints invite surprise upgrades that break production; a missing or stale lockfile means every `composer install` resolves different versions; autoload misconfiguration means classes that work in dev but not in prod; and a dependency conflict resolved by forking or patching creates a maintenance surface that outlives the original problem.

Agents often treat Composer as "run install, copy the vendor folder," and the harm is delayed until deployment or an upgrade. A caret constraint (`^1.2`) that silently pulled a 1.9 with a regression; a `composer.lock` that was not committed so the deploy server resolved fresh versions and got a different patch; a PSR-4 namespace that does not match the directory structure so the class is not found in production; a dev dependency that leaked into production because `install` was run without `--no-dev`. The judgment problem is to choose constraints that express real compatibility, to commit and respect the lockfile, to configure autoload to match the actual structure, to separate dev-only tooling, and to make the build reproducible from a clean checkout.

## Core Rules

### Choose Version Constraints To Express Real Compatibility

Composer constraints are how you tell the resolver what versions are acceptable. The constraint is a compatibility statement, not a wish; too loose and you get surprise breakage, too tight and you block legitimate updates and security patches.

- **Caret (`^1.2.3`)** allows `>=1.2.3` and `<2.0.0` — backwards-compatible changes within the same major, per semver. This is the right default for libraries that follow semver, because it picks up bugfixes and features without breaking changes.
- **Tilde (`~1.2.3`)** allows `>=1.2.3` and `<1.3.0` — patch updates only. Use when you want to be stricter than caret, e.g., for libraries with a history of breaking in minor releases.
- **Exact (`1.2.3`)** pins to one version. Rarely the right choice for a library dependency; occasionally right for a specific known-good patch you cannot risk changing.
- **Range or wildcard (`>=1.2 <3.0`, `1.*`)** is looser and riskier; use only when you have a specific reason and understand the resolver may pick versions you have not tested.
- For your own application (not a library), you can afford tighter constraints because you do not need to support a range of downstream versions; for a library you publish, prefer caret to let consumers update.

The key question: if the resolver picks the highest version satisfying the constraint, will it work? If you are not sure, tighten the constraint or test against the highest version.

### Commit And Respect composer.lock

The lockfile records the exact resolved versions of every dependency, direct and transitive. It is what makes `composer install` reproducible across machines and deployments.

- Commit `composer.lock` for applications. A deploy from a clean checkout must produce the same versions that were tested. Without the lockfile, `composer install` resolves fresh and may pick newer versions that were never tested.
- Run `composer update` deliberately, not as part of routine install. `update` re-resolves to the newest versions satisfying the constraints and rewrites the lockfile; it is a deliberate upgrade action, reviewed and tested. `install` reads the lockfile and reproduces it.
- For libraries, the convention is often to not commit the lockfile (so consumers resolve their own), but commit it if you want CI to test the exact versions you develop against; document the choice.

Strong choice: lockfile committed, CI runs `composer install` (lockfile-faithful), upgrades are deliberate `composer update` PRs. Weak choice: lockfile gitignored, every deploy runs `composer update`, production gets untested versions.

### Configure PSR-4 Autoload To Match The Real Structure

PSR-4 maps a namespace prefix to a directory, so `Acme\Foo\Bar` maps to `src/Foo/Bar.php` for the `"Acme\\": "src/"` rule. Getting this wrong means classes work in some contexts and not others, or the autoloader falls back to a slow path.

- The namespace prefix in `composer.json` must match the actual namespace declarations in your classes, and the directory must match where the files live. A mismatch (namespace `Acme\Foo`, file at `src/Bar.php`, rule `"Acme\\Foo\\": "src/"` expecting `src/Bar.php` to be `Acme\Foo\Bar`) causes "class not found" errors.
- One prefix per root; do not overlap prefixes in ways that confuse the resolver. Use `"Acme\\": "src/"` and put everything under `src/` with matching namespaces.
- After changing autoload rules, run `composer dump-autoload` (or `composer install`, which dumps as well) so the optimized autoloader is regenerated. In production, use `--optimize-autoloader` (or `--classmap-authoritative`) for faster class lookup.
- Use `autoload-dev` for test-only namespaces so test classes are not autoloaded in production.

### Separate require From require-dev

`require` is for production dependencies; `require-dev` is for development tooling (test frameworks, static analyzers, linters, fixtures). Mixing them bloats production and can pull in tools that should never run there.

- Test frameworks (PHPUnit, Pest), static analyzers (PHPStan, Psalm), linters, and code-style tools belong in `require-dev`.
- Production-only deps, including PSR interfaces you implement and libraries your runtime code calls, belong in `require`.
- Deploy and CI production images should install with `composer install --no-dev` to exclude dev dependencies; dev/CI runs install everything.

### Resolve Conflicts By Understanding, Then Patch Or Fork Deliberately

Dependency conflicts (two packages requiring incompatible versions of a third) are common in PHP, especially across major framework versions. The resolution should be deliberate, not a hack.

- First, understand the conflict: `composer why-not package:version` and `composer why dependency` show which packages require what. Often the fix is to update or downgrade a direct dependency so the transitive constraints align.
- Avoid inline aliases (`dev-master as 1.5`) and `provide` hacks unless you fully understand them; they tell the resolver a version is compatible when it may not be, and the breakage surfaces at runtime.
- If a dependency has a bug and you need a patched version, prefer a fork with a real version tag over an inline patch; document the fork and the reason, and track upstream so you can return to the canonical package when fixed.
- For long-lived forks, consider maintaining them as a published package or contributing the fix upstream; an untracked fork is technical debt that grows.

### Make The Build Reproducible From A Clean Checkout

A new developer, a CI runner, and a deploy server should all get the same result from `composer install`. Anything that requires manual steps, local state, or "it works on my machine" knowledge is a defect.

- The lockfile, the autoload config, the PHP version constraint (`"php": ">=8.1"`), and required PHP extensions (`"ext-*"`) should all be declared. A missing extension declaration fails loudly on the deploy server; a missing PHP constraint lets a newer PHP with breaking changes run your code.
- Pin the PHP version range to what you test against; do not leave it loose and discover at deploy that a PHP 8.4 deprecation breaks your code.
- Use `composer check-platform-requirements` in CI to catch missing extensions before deploy.

## Common Traps

### Loose Constraints Pulling Surprise Upgrades

`"guzzlehttp/guzzle": "*"` or `"^1.0"` on a library that has since released a breaking 2.0 will, on the next `composer update`, pull a version your code does not support. Use caret on semver-following libraries, and test against the highest satisfying version.

### Lockfile Not Committed, Deploy Resolves Fresh

Without the lockfile, `composer install` falls back to `composer update` semantics, resolving the newest satisfying versions. Production gets versions that were never tested. Commit the lockfile for applications.

### Running composer update In Deploy

A deploy script that runs `composer update` re-resolves and may change versions. Deploy with `composer install --no-dev` against the committed lockfile; reserve `update` for deliberate upgrades.

### PSR-4 Namespace Mismatch

A namespace prefix that does not match the directory structure causes "class not found" in contexts that do not have a fallback autoloader. Verify the mapping with `composer dump-autoload` and a class lookup; mismatches are silent until a specific class is first referenced.

### Dev Dependencies In Production

Installing with `composer install` (no `--no-dev`) in a production image pulls PHPUnit, PHPStan, and test fixtures into the production environment, increasing image size and attack surface. Use `--no-dev` (and `--optimize-autoloader`) for production installs.

### Inline Alias Hiding Incompatibility

`"package": "dev-bugfix as 1.5.0"` tells the resolver a dev branch is compatible with a stable constraint, but the branch may have breaking changes. Use only when you have verified compatibility, and document why.

### Forgetting ext-* And PHP Version Requirements

A package using the `intl` extension or PHP 8.2 enums deployed to a server without them fails at runtime with a confusing error. Declare `"ext-intl": "*"` and `"php": ">=8.2"` so `composer check-platform-requirements` catches it.

### Stale Lockfile After Constraint Changes

Editing `composer.json` constraints without running `composer update <package>` leaves the lockfile out of sync; the next `composer install` warns or fails. After changing constraints, update the affected package and commit both files together.

## Self-Check

- [ ] Version constraints express real compatibility (caret as the default for semver libraries, tighter for risk-averse applications), and the code has been tested against the highest satisfying version.
- [ ] `composer.lock` is committed for applications, CI runs `composer install` against it, and `composer update` is a deliberate, reviewed upgrade action.
- [ ] PSR-4 autoload mappings match the actual namespace declarations and directory structure, `autoload-dev` holds test-only namespaces, and production uses an optimized autoloader.
- [ ] `require` holds production dependencies only; test frameworks, analyzers, and linters are in `require-dev`; production installs use `--no-dev`.
- [ ] Dependency conflicts are understood before resolution, inline aliases and `provide` hacks are rare and documented, and forks are tracked with a path back to upstream.
- [ ] The PHP version constraint and required `ext-*` extensions are declared, and `composer check-platform-requirements` passes in CI.
- [ ] A clean checkout reproduces the tested versions via `composer install` with no manual steps or local-state dependencies.
- [ ] No `composer update` runs in deploy or CI install steps; updates are isolated to deliberate upgrade PRs.
- [ ] Platform requirements (PHP version, extensions) match the production runtime, and a missing requirement fails at install rather than at runtime.
- [ ] Any forked or patched dependency has a documented reason and a plan to return to the canonical package when upstream fixes the issue.
