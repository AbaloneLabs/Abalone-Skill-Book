---
name: spring_and_dependency_injection.md
description: Use when the agent is writing or reviewing Spring or Spring Boot applications, configuring dependency injection (constructor vs field vs setter injection), defining beans and component scanning, choosing bean scopes, using @Configuration/@Bean/@Component/@Service/@Repository, profiles and conditional beans, Spring's annotation-driven model (@Autowired, @Qualifier, @Primary), transaction management (@Transactional), Spring Boot auto-configuration, or diagnosing ambiguous bean errors, circular dependencies, scope/proxying bugs, proxy-not-working issues with @Transactional or @Async, context startup failures, or over-engineered DI graphs. Covers IoC container semantics, bean lifecycle, proxy mechanics, and the discipline of keeping the DI graph testable and explicit.
---

# Spring And Dependency Injection

Spring's core is an Inversion-of-Control container that constructs and wires your objects (beans) according to declarations, and Spring Boot layers auto-configuration on top so that a runnable application emerges from a handful of annotations. The promise is that you write small, decoupled, testable components and the container assembles them; the reality is that the container is a runtime program with lifecycle, scoping, proxying, and conditional logic whose behavior is invisible until it breaks. A circular dependency throws `BeanCurrentlyInCreationException` at startup; a `@Transactional` method called from another method in the same class silently runs without a transaction because the proxy is bypassed; a field-injected field makes the class un-testable without the container; an auto-configuration pulls in a bean you did not ask for and changes behavior. Spring makes correct architecture easy and subtle architecture bugs trivial to write.

The judgment problem is not "how do I add `@Service`" but "what is the actual dependency graph, is each dependency explicit and injectable, what scope and lifecycle does each bean have, and where does the proxy boundary matter." Agents tend to over-annotate (every class a bean, every field `@Autowired`), to rely on field injection (which hides dependencies and blocks constructor testing), to ignore bean scopes (a singleton holding a request-scoped collaborator without a proxy fails or leaks), and to assume annotations are magic rather than proxies and lifecycle callbacks. Each produces code that starts up green and fails in a specific, hard-to-debug way under real conditions.

## Core Rules

### Prefer Constructor Injection; Make Dependencies Explicit And Final

Constructor injection is the default and the strongest form of dependency injection: a bean declares its collaborators as parameters of its constructor, and the container injects them when constructing the bean. This makes the dependency graph explicit (the constructor signature is the list of collaborators), makes the fields `final` (the collaborators cannot be reassigned, which is a correctness property), and makes the class testable without the container (you construct it with `new` and fakes in a unit test). With a single constructor, Spring 4.3+ does not even require `@Autowired`. This is the form to reach for by default.

Field injection (`@Autowired` on a private field) is the weak form: it hides the dependency list (the class looks like it has no collaborators until you scan every field), makes the field non-final and mutable, and forces every test to use reflection or the container because there is no constructor to call. Setter injection is a middle ground, justified only for optional dependencies or circular-structure cases. The discipline: if a dependency is required (most are), it is a constructor parameter and a final field. Reserve field/setter injection for the rare optional or circular case, and never use field injection because the IDE quick-fix suggested it. A class whose dependencies are not visible in its constructor is a class that is hard to test and hard to reason about.

### Keep The Bean Graph Acyclic; Resolve Or Eliminate Circular Dependencies

A circular dependency (A depends on B depends on A) is a structural problem that the container will surface as `BeanCurrentlyInCreationException` for constructor injection, because neither bean can be constructed without the other. This is the container telling you the design is wrong: two beans that mutually depend on each other are usually one bean split incorrectly, or they share a third dependency that should be extracted. The fix is almost always to refactor the cycle away — extract the shared logic into a third bean that both depend on, or merge the two beans, or break the cycle with an event/callback instead of a direct call.

Field and setter injection can mask cycles (Spring resolves them with early bean references), but masking is not fixing — the cycle still exists, the beans are still tightly coupled, and the masked version is harder to test and reason about. Modern Spring (6+, Boot 3) refuses even setter/field cycles by default and requires `spring.main.allow-circular-references=true` to permit them, which is a signal that cycles are considered a defect. Do not enable that flag as a quick fix; resolve the cycle. If you genuinely need a circular reference (rare, usually a callback registration), use `@Lazy` on one side to break the construction-time cycle, and document why.

### Choose Bean Scopes Deliberately; Understand Singleton-Injecting-Scoped

Spring beans have scopes, and the default is singleton (one instance per container). Other scopes — prototype (a new instance every time it is injected/looked up), request (one per HTTP request), session (one per HTTP session), and application — change the lifecycle and have a critical interaction with singleton injection. A singleton bean that injects a request-scoped bean needs a proxy (Spring injects a proxy that delegates to the current request's instance); without the proxy (or with `scopedProxy.mode = NO`), the singleton captures one request's instance and reuses it forever, leaking request state across requests and producing heisenbugs.

The rule: know the scope of every bean, and when a longer-lived bean depends on a shorter-lived one, use a scoped proxy or inject a provider (`ObjectProvider<T>`, `Provider<T>`) so the shorter-lived instance is resolved at use time, not at construction. Prototype-scoped beans injected into a singleton are injected once (at construction) unless you use a provider — a common surprise where "prototype" did not mean "new instance every call." Stateful beans (a per-request context, a per-user cache) should not be singletons; stateless services should be singletons. Mixing them up — a singleton holding mutable per-request state — is a concurrency and correctness bug.

### Understand That Spring Annotations Are Proxies, And Respect The Proxy Boundary

Most of Spring's powerful annotations (`@Transactional`, `@Async`, `@Cacheable`, `@EventListener`, `@PreAuthorize`) work by wrapping the bean in a dynamic proxy (JDK interface proxy or CGLIB subclass proxy) that intercepts the method call and adds behavior. This means the behavior is applied only when the call goes through the proxy — i.e., when the method is called from outside the bean. A public method in a Spring bean that calls another `@Transactional` method on the same bean (via `this`) bypasses the proxy: the second method runs without its transaction, without its async dispatch, without its cache check. This "self-invocation" problem is the single most common Spring bug, and it is silent — no error, just missing behavior.

The fixes are structural. Extract the collaborator into a separate bean so the call crosses the proxy boundary. Or, rarely, inject the bean into itself (`@Autowired private MyService self;` and call `self.otherMethod()`) so the call goes through the proxy — but this is a smell that usually indicates the methods belong in different beans. Also respect proxy mechanics: `@Transactional` and friends do not work on `private`, `static`, or `final` methods (proxies cannot intercept them), and they do not work on calls from within the same class. If a transaction or async or cache annotation "isn't doing anything," the first thing to check is self-invocation or method visibility. The annotation is not magic; it is a proxy, and proxies have boundaries.

### Make @Transactional Precise: Boundaries, Propagation, Rollback, And Read-Only

`@Transactional` demarcates a database transaction, and its defaults are sensible for the common case but wrong for several common cases. By default: propagation is REQUIRED (join an existing transaction or start a new one), rollback is for unchecked exceptions (RuntimeException and Error, not checked exceptions), and the transaction is read-write. The traps are concrete. A `@Transactional` method that catches its own exception and returns normally will not roll back, because rollback is triggered by the exception propagating to the proxy — catching it defeats rollback. A method you intend as read-only that is not marked `readOnly = true` takes a write transaction (and on some databases acquires a write lock). A checked exception you want to roll back on is not in the default rollback set, so you need `rollbackFor = MyCheckedException.class`.

Place `@Transactional` at the service layer (the boundary of a unit of business work), not on every method, and not on data-access methods (the repository already runs in the caller's transaction). Keep transactional methods short and focused on the unit of work; a transaction that spans a remote call or a long computation holds database resources and locks for the duration. Understand propagation: REQUIRED (default) joins an outer transaction, so an inner failure rolls back the whole outer work; REQUIRES_NEW suspends the outer and starts a fresh transaction (use deliberately, it holds two connections); NESTED uses a savepoint. Choose propagation knowingly, because the default (REQUIRED) means an inner `@Transactional` does not create a new transaction — it joins the caller's.

### Use Spring Boot Auto-Configuration As Convention, Not Magic

Spring Boot's auto-configuration inspects the classpath and configuration properties and creates beans you did not declare — a datasource when a JDBC driver is present, an entity manager when JPA is present, a web server when the web starter is present. This is enormously productive and also opaque: a bean appears because a condition matched, and when behavior changes unexpectedly, the cause is often an auto-configuration that activated or deactivated based on a classpath change. Use the actuator's `/actuator/conditions` (or `/autoconfig`) endpoint to see which auto-configurations matched and why; do not guess.

The discipline: treat auto-configuration as a set of sensible defaults you can override, not as an inscrutable black box. When you need different behavior, define the bean explicitly (your bean overrides the auto-configured one for most auto-configurations), or exclude a specific auto-configuration (`@SpringBootApplication(exclude = ...)`, or `spring.autoconfigure.exclude` in properties), or use `@Conditional*` annotations in your own `@Configuration` to mirror the pattern. Keep your own configuration in `@Configuration` classes (not scattered `@Component` beans) so the wiring is readable. Profile your beans (`/actuator/beans`) when the graph is unclear. A Spring Boot app whose startup behavior is not understood is an app that will surprise you in production.

### Keep The DI Graph Testable And Avoid Over-Wiring

The point of DI is testability and decoupling, and the test is whether a bean can be unit-tested without the container. A bean with constructor injection and final fields can be instantiated with fakes in a plain JUnit test; a bean with field injection and a dozen `@Autowired` fields cannot. Design beans so that the collaborators are interfaces (or at least injectable types), so fakes are easy, and so the bean's logic is testable in isolation. Integration tests then use the container (`@SpringBootTest`) to verify the wiring, but the unit tests should not need it.

Avoid over-wiring: a bean with 15 collaborators is doing too much and should be split; a graph so deep that startup is slow or so tangled that changes ripple everywhere is a sign of poor module boundaries. Prefer composition of small beans over god-beans. Prefer interfaces for collaborators so implementations can be swapped in tests and so modules are decoupled. And resist the urge to make everything a bean — plain objects constructed with `new` are correct for value objects, DTOs, and helpers that have no dependencies and no lifecycle; not every class needs to be container-managed. The container manages beans that have dependencies, lifecycle, or cross-cutting behavior; everything else is a plain object.

## Common Traps

### Field Injection Hiding Dependencies And Blocking Testing

`@Autowired` on private fields makes the dependency list invisible and the class un-testable without reflection or the container. Use constructor injection with final fields.

### Self-Invocation Bypassing @Transactional/@Async/@Cacheable

A bean method calling another annotated method on `this` bypasses the proxy, so the annotation has no effect. Extract to a separate bean or inject self through the proxy.

### @Transactional On Private, Static, Or Final Methods

Proxies cannot intercept these, so the annotation silently does nothing. Use public, non-final methods on Spring-managed beans.

### Catching An Exception Inside @Transactional Defeats Rollback

A method that catches its exception and returns normally does not roll back, because rollback is triggered by propagation to the proxy. Re-throw, or call `TransactionAspectSupport.currentTransactionStatus().setRollbackOnly()`.

### Singleton Holding A Request/Session-Scoped Bean Without A Proxy

A singleton that injects a request-scoped bean without a scoped proxy captures one request's instance forever, leaking state across requests. Use a scoped proxy or an `ObjectProvider`.

### Prototype Bean Injected Into A Singleton Behaving As Singleton

A prototype bean injected into a singleton is created once (at injection), not per call. Use `ObjectProvider<T>` or method-level lookup to get a new instance each time.

### Circular Dependencies "Fixed" With allow-circular-references

Enabling circular references masks a structural defect. Refactor the cycle (extract shared logic, merge beans, use events); use `@Lazy` only with a documented reason.

### Over-Annotating Every Class As A Bean

Making value objects, DTOs, or stateless helpers into container beans adds wiring cost for no benefit. Manage beans that have dependencies, lifecycle, or cross-cutting behavior; use plain objects for the rest.

### Relying On Auto-Configuration Without Understanding It

A classpath change silently activates or deactivates an auto-configuration, changing behavior. Inspect `/actuator/conditions`; override or exclude explicitly when behavior matters.

## Self-Check

- [ ] Required dependencies use constructor injection with `final` fields; field injection is absent except for documented optional or circular cases, and every bean is unit-testable without the container.
- [ ] The bean dependency graph is acyclic; any cycle was refactored away (or broken with `@Lazy` for a documented callback reason), and `spring.main.allow-circular-references` is not enabled as a mask.
- [ ] Every bean's scope is known and deliberate; longer-lived beans that depend on shorter-lived (request/session/prototype) collaborators use scoped proxies or `ObjectProvider`/`Provider` so the instance is resolved at use time, not captured at construction.
- [ ] Proxy-based annotations (`@Transactional`, `@Async`, `@Cacheable`, security/`@PreAuthorize`) are applied to public, non-final methods on Spring-managed beans, and no annotated behavior is bypassed by self-invocation.
- [ ] `@Transactional` is placed at the service boundary with deliberate propagation (REQUIRED joins the caller; REQUIRES_NEW holds two connections), `readOnly = true` on read paths, `rollbackFor` covering checked exceptions, and exceptions are not caught-and-swallowed inside the transactional method.
- [ ] Spring Boot auto-configuration is understood (inspectable via `/actuator/conditions`), overridden or excluded explicitly where behavior must differ, and the team's own wiring lives in readable `@Configuration` classes rather than scattered beans.
- [ ] Beans are small and focused (no 15-collaborator god-beans), collaborators are interfaces where it aids testing and decoupling, and value objects/DTOs/helpers without dependencies are plain objects, not container beans.
- [ ] The DI graph was verified both ways: unit tests construct beans with fakes (no container), and integration tests (`@SpringBootTest`) verify the wiring and startup; context startup failures and ambiguous-bean errors are resolved at the design level, not suppressed.
