# cabidl
Component Architecture Boundary and Interface Definition Language

---

# What is CABIDL?

**CABIDL** stands for **Component Architecture Boundary and Interface Definition Language**.

CABIDL is a textual software architecture language that describes a system in terms of:

- **Components** — the fundamental building blocks of your software
- **Interfaces** — what each component provides or requires
- **Boundaries** — the separation between your system and the outside world
- **Implementation assignment** — the realization or mapping of components
- **Relationships and connections** — how components interact and fit together
- **The overall architecture** — captured in a single, readable source file

The purpose of CABIDL is to offer a **single location** where the entire software architecture is described **clearly and explicitly**. Instead of relying on disconnected diagrams, wikis, conventions, and tribal knowledge, CABIDL provides a single textual representation of the system’s structure and points of access.

A CABIDL file typically answers:

- What are the major components in this system?
- What does each component expose?
- What dependencies does each component have?
- Which interfaces are internal and which are externally accessible?
- What are the external entry points or boundary surfaces?
- How are components wired together?
- What implementation backs a given component?
- What’s inside the system boundary, and what’s outside?

CABIDL is **not just an interface language and not just a deployment language**. It’s an architecture definition language focused on **components, boundaries, interfaces, and composition**.

> **Concise definition:**  
> CABIDL is a textual language for defining software components, their interfaces, implementations, boundaries, and relationships in a single architecture specification.

---

# When to Use CABIDL

**Use CABIDL when you need your system’s architecture to be:**
- Explicit
- Reviewable
- Maintainable as code

CABIDL is especially valuable when informal diagrams and scattered documentation are no longer sufficient.

**Scenarios where CABIDL is a great fit:**

- **Your system has multiple components**:  
  If your software includes services, modules, subsystems, adapters, gateways, workers, APIs, or processors that interact in nontrivial ways, CABIDL describes how these parts fit together.
- **Interfaces matter**:  
  When you need to define exactly what a component provides and consumes, especially where interface contracts drive system integrity.
- **Boundaries matter**:  
  When you need to spell out external APIs, ingress/egress points, trusted boundaries, or integration zones with third parties—CABIDL excels.
- **Architecture should live in version control**:  
  CABIDL files are easy to diff, review, store alongside code, validate, and use as input to tooling.
- **A shared architectural source of truth is required**:  
  For multi-team or multi-developer environments, CABIDL removes ambiguity about structure, ownership, and exposure.
- **You want to generate artifacts from architecture**:  
  CABIDL definitions can enable generation of architecture diagrams, interface inventories, dependency maps, documentation, validation rules, scaffolding, or conformance checks.

---

# Where CABIDL Shines

CABIDL is particularly strong for:

- Service-oriented systems
- Modular monoliths
- Microservice platforms
- Plugin-based applications
- Distributed systems
- Systems requiring strict API or boundary governance
- Environments needing architectural review and traceability

It fills a practical middle ground between very informal docs and heavyweight modeling frameworks.

---

# When Not to Use CABIDL

CABIDL may not be necessary if your system is **small, short-lived, or structurally trivial** (e.g., a single script or simple app). If explicit architectural boundaries are absent, CABIDL may add more overhead than benefit.

It is **not a replacement for**:

- Detailed code or implementation
- Protocol schemas
- Deployment manifests
- Runtime observability
- Low-level documentation

CABIDL’s purpose is to define architecture, not to cover all engineering details.

---

# CABIDL in a Nutshell

> CABIDL is a textual architecture definition language for describing software systems as components with explicit interfaces, boundaries, implementations, and relationships. Use it when your system structure must be defined clearly in a single file and maintained as a source-controlled artifact.

---