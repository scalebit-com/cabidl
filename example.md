
# ECommerce Platform

**CABIDL Specification**  
Version: 1.5  
Last Updated: 2026-03-21  
Status: Approved

## System Overview
This file is the single source of truth for the entire architecture.  
It defines boundaries, interfaces, components, implementations and wiring in one readable document.

---

## Boundary: PublicAPI

```yaml
name: PublicAPI
kind: boundary
description: Externally accessible entry points (internet-facing)
exposes:
  - REST_API
  - WebFrontend
trust_level: public
```

The PublicAPI boundary is the only part of the system exposed to the internet. All external clients (mobile apps, browsers, partners) must go through the ApiGateway that lives inside this boundary.

---

## Boundary: TrustedZone

```yaml
name: TrustedZone
kind: boundary
description: Internal services running in the private VPC
exposes: []
trust_level: internal
```

All domain services live inside the TrustedZone. They can freely call each other but are never directly reachable from the outside world.

---

## Boundary: ExternalServices

```yaml
name: ExternalServices
kind: boundary
description: Third-party SaaS providers we integrate with
exposes: []
consumes:
  - StripePaymentGateway
  - SendGridEmail
trust_level: external
```

Third-party services we depend on but do not control.

---

## Interface: UserAPI

```yaml
name: UserAPI
kind: interface
provides:
  - registerUser(RegisterRequest) -> UserResponse
  - authenticate(AuthRequest) -> TokenResponse
  - getProfile(UserId) -> Profile
version: 2.1
```

This is the contract for all user-related operations. Every implementation that claims to provide `UserAPI` must respect this exact contract.

---

## Interface: CatalogAPI

```yaml
name: CatalogAPI
kind: interface
provides:
  - listProducts(Category?, Page) -> ProductList
  - getProduct(ProductId) -> ProductDetails
version: 1.0
```

Public catalog interface used by both the frontend and the order service.

---

## Interface: OrderAPI

```yaml
name: OrderAPI
kind: interface
provides:
  - placeOrder(OrderRequest) -> OrderConfirmation
  - getOrderStatus(OrderId) -> OrderStatus
version: 1.3
```

Core ordering contract.

---

## Interface: PaymentProcessing

```yaml
name: PaymentProcessing
kind: interface
provides:
  - processPayment(PaymentRequest) -> PaymentResult
version: 1.0
```

Internal payment abstraction (implemented by PaymentService).

---

## Component: ApiGateway

```yaml
name: ApiGateway
kind: component
provides:
  - PublicAPI.REST_API
requires:
  - UserAPI
  - CatalogAPI
  - OrderAPI
implementation: gateway.ApiGatewayImpl
technology: Kotlin + Ktor
deployed_in: PublicAPI
```

The ApiGateway is the single entry point for all external traffic. It authenticates requests, routes them to the appropriate internal services, and applies rate limiting and CORS.

---

## Component: UserService

```yaml
name: UserService
kind: component
provides:
  - UserAPI
requires:
  - Database.Postgres
implementation: users.UserDomainService
technology: Java + Spring Boot
deployed_in: TrustedZone
```

Owns all user data and authentication logic. This is the only component allowed to write to the users table.

---

## Component: CatalogService

```yaml
name: CatalogService
kind: component
provides:
  - CatalogAPI
requires:
  - Database.Postgres
  - SearchEngine.Elastic
implementation: catalog.CatalogDomainService
technology: Go
deployed_in: TrustedZone
```

Manages product catalog and search indexing.

---

## Component: OrderService

```yaml
name: OrderService
kind: component
provides:
  - OrderAPI
requires:
  - UserAPI
  - CatalogAPI
  - PaymentProcessing
implementation: orders.OrderDomainService
technology: TypeScript + NestJS
deployed_in: TrustedZone
```

Orchestrates the entire order lifecycle.

---

## Component: PaymentService

```yaml
name: PaymentService
kind: component
provides:
  - PaymentProcessing
requires:
  - ExternalServices.StripePaymentGateway
implementation: payments.PaymentDomainService
technology: Python + FastAPI
deployed_in: TrustedZone
```

Handles all payment interactions (never stores card data).

---

## Wiring & Relationships

```yaml
kind: connections
connections:
  - ApiGateway.UserAPI -> UserService
  - ApiGateway.CatalogAPI -> CatalogService
  - ApiGateway.OrderAPI -> OrderService
  - OrderService.PaymentProcessing -> PaymentService
  - OrderService.UserAPI -> UserService
  - OrderService.CatalogAPI -> CatalogService
```

All wiring is explicitly declared here. No implicit dependencies allowed.

---

**End of CABIDL specification**  
This file can be parsed by tools to generate diagrams, validation reports, interface inventories, or scaffolding code.


