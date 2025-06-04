# Decision Log - Loco Platform

## 📝 Format
Each decision follows this structure:
- **Date**: When the decision was made
- **Decision**: What was decided
- **Context**: Why this decision was needed
- **Alternatives Considered**: Other options evaluated
- **Rationale**: Why this option was chosen
- **Impact**: Expected effects on the project

---

## 2025-01-06: Choose Dioxus for Frontend Framework

**Decision**: Use Dioxus as the primary frontend framework

**Context**: Need a Rust-based frontend framework that compiles to WebAssembly for the Loco Platform

**Alternatives Considered**:
1. **Yew**: Mature, large ecosystem, Elm-inspired
2. **Leptos**: Fine-grained reactivity, good performance
3. **Sycamore**: Solid.js-inspired, reactive
4. **Percy**: Virtual DOM approach

**Rationale**: 
- Dioxus offers React-like syntax familiar to many developers
- Excellent cross-platform support (web, desktop, mobile)
- Active development and community
- Good documentation and examples
- Built-in router and state management

**Impact**: 
- Faster developer onboarding due to familiar patterns
- Potential for future mobile/desktop apps
- Need to work around some WASM limitations

---

## 2025-01-06: Implement Demo Mode for Backend

**Decision**: Create demo mode with mock data instead of requiring database for initial development

**Context**: Build issues and complexity of setting up full database infrastructure blocking progress

**Alternatives Considered**:
1. **SQLite in-memory**: Simpler than PostgreSQL but still needs migrations
2. **Docker Compose**: Full PostgreSQL but adds complexity
3. **Embedded database**: Like sled or RocksDB

**Rationale**:
- Allows immediate testing of API endpoints
- Reduces initial setup complexity
- Enables frontend development without database
- Can gradually migrate to real database

**Impact**:
- ✅ Faster initial development
- ✅ Easier onboarding for new developers
- ⚠️ Need to ensure demo data matches real schema
- ⚠️ Must plan migration path to real database

---

## 2025-01-06: Fix Duplicate Dependencies First

**Decision**: Resolve all Cargo dependency conflicts before continuing feature development

**Context**: Multiple duplicate dependencies (web-sys, tower-http) preventing compilation

**Alternatives Considered**:
1. **Vendor dependencies**: Copy and modify locally
2. **Fork and patch**: Maintain custom versions
3. **Downgrade versions**: Use older compatible versions

**Rationale**:
- Clean builds are essential for development velocity
- Dependency conflicts cause subtle bugs
- Rust's strict compiler requires resolution
- Sets good foundation for project

**Impact**:
- ✅ All crates now compile successfully
- ✅ Consistent dependency versions
- ✅ Reduced future compatibility issues

---

## 2024-12-20: Comprehensive Map Implementation

**Decision**: Build advanced map features with 50+ reactive signals and Apple-style UI

**Context**: Map visualisation is core differentiator for job discovery

**Alternatives Considered**:
1. **Basic markers only**: Simpler but less engaging
2. **List view only**: No geographic context
3. **Third-party embed**: Limited customisation

**Rationale**:
- Visual job discovery improves user engagement
- Location-based search is primary use case
- Competitive advantage over text-only platforms
- Australian geography suits map-based approach

**Impact**:
- ✅ Significantly enhanced user experience
- ✅ Differentiated product offering
- ⚠️ Increased complexity to maintain
- ⚠️ Performance considerations for mobile

---

## 2025-01-06: SeaORM for Database ORM

**Decision**: Use SeaORM as the primary database ORM

**Context**: Need async-first ORM for PostgreSQL integration

**Alternatives Considered**:
1. **Diesel**: Mature, sync-only without diesel-async
2. **SQLx**: Type-safe raw SQL, no ORM features
3. **Raw tokio-postgres**: Maximum control, more boilerplate

**Rationale**:
- Built for async Rust from ground up
- Dynamic query building capabilities
- Good migration system
- Active development
- Clean API design

**Impact**:
- Async queries throughout application
- Type-safe database operations
- Learning curve for team
- Good documentation available

---

## 2025-01-06: Australian English Everywhere

**Decision**: Use Australian English spelling and terminology throughout codebase

**Context**: Building for Australian market requires local language consistency

**Alternatives Considered**:
1. **US English**: More common in tech
2. **UK English**: Closer to Australian
3. **Mixed**: Use US for code, AU for content

**Rationale**:
- Consistency across entire project
- Shows commitment to local market
- Reduces confusion for Australian team
- Better for Australian users

**Impact**:
- `colour` not `color` in variables
- `organisation` not `organization`
- `licence` not `license`
- Need spell checker configuration

---

## 2025-01-06: JWT for Authentication

**Decision**: Implement JWT-based authentication with refresh tokens

**Context**: Need stateless authentication for scalability

**Alternatives Considered**:
1. **Session cookies**: Server state required
2. **OAuth only**: Complex for simple use cases
3. **API keys**: Less secure for web clients

**Rationale**:
- Stateless allows horizontal scaling
- Industry standard for SPAs
- Works well with mobile apps
- Good library support in Rust

**Impact**:
- Client-side token storage needed
- Refresh token rotation complexity
- Clear security model
- Enables microservices future

---

## 2025-01-06: Monorepo Structure

**Decision**: Use Cargo workspace monorepo for all code

**Context**: Need to share types between frontend and backend

**Alternatives Considered**:
1. **Separate repos**: More complex coordination
2. **Git submodules**: Difficult to manage
3. **Published crates**: Overhead for private code

**Rationale**:
- Atomic commits across stack
- Shared type safety
- Easier refactoring
- Single CI/CD pipeline
- Simplified development

**Impact**:
- ✅ Type sharing works perfectly
- ✅ Consistent versioning
- ⚠️ Larger repository size
- ⚠️ Need good workspace tooling

---

## 2025-01-06: TailwindCSS for Styling

**Decision**: Use TailwindCSS utility classes for component styling

**Context**: Need consistent, maintainable styling system

**Alternatives Considered**:
1. **CSS Modules**: More traditional separation
2. **Styled Components**: Runtime overhead in WASM
3. **Vanilla CSS**: Too much boilerplate
4. **Stylist**: Rust CSS-in-RS

**Rationale**:
- Rapid development with utilities
- Consistent design system
- Small bundle with PurgeCSS
- Great documentation
- Works well with components

**Impact**:
- HTML can be verbose
- Learning curve for team
- Very fast styling iteration
- Mobile-first by default

---

## 2025-01-06: Phase-Based Development

**Decision**: Implement features in strict phases per checklist.md

**Context**: Large project needs structured approach

**Alternatives Considered**:
1. **Feature branches**: Less coordinated
2. **Agile sprints**: Less comprehensive
3. **Waterfall**: Too rigid

**Rationale**:
- Clear priorities and dependencies
- Measurable progress
- Reduces scope creep
- Allows partial deployments
- Better resource planning

**Impact**:
- Structured development flow
- Clear success metrics
- Some features delayed
- Better project visibility

---

## Future Decisions to Make

### Pending Decisions
1. **Payment Processing**: Stripe vs Australian alternatives
2. **Email Service**: SendGrid vs AWS SES vs Postmark  
3. **File Storage**: S3 vs Cloudflare R2 vs local
4. **Search Engine**: PostgreSQL FTS vs Elasticsearch vs Meilisearch
5. **Mobile Strategy**: PWA vs React Native vs Native
6. **Deployment Platform**: AWS vs GCP vs Azure vs Bare Metal
7. **CDN Provider**: Cloudflare vs Fastly vs AWS CloudFront
8. **Monitoring**: Datadog vs New Relic vs Open Source stack
9. **Analytics**: Plausible vs Matomo vs Custom
10. **A/B Testing**: Build vs Buy solution

### Decision Criteria
- **Technical Fit**: Integrates well with Rust/WASM
- **Cost**: Sustainable for startup budget
- **Scalability**: Can handle growth
- **Australian Compliance**: Privacy laws, data residency
- **Developer Experience**: Good documentation, tooling

---

**Last Updated**: 2025-01-06
**Next Review**: When facing next major technical decision