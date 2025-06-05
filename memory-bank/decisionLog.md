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

## 2025-01-06: MAJOR DECISION - Migration from Dioxus to Leptos

**Decision**: Migrate from Dioxus to Leptos as the primary frontend framework

**Context**: After initial implementation with Dioxus, discovered superior performance and reactivity benefits of Leptos for our use case

**Original Choice (Dioxus)**:
1. **Yew**: Mature, large ecosystem, Elm-inspired
2. **Leptos**: Fine-grained reactivity, good performance
3. **Sycamore**: Solid.js-inspired, reactive
4. **Percy**: Virtual DOM approach

**Migration Rationale**:
- **Fine-grained Reactivity**: Leptos only updates changed DOM nodes, not entire component trees
- **Better Performance**: Faster hydration, smaller bundle sizes, more efficient updates
- **Superior SSR/CSR**: Seamless integration between server and client rendering
- **Modern Architecture**: More sophisticated signal system than Fermi atoms
- **Growing Ecosystem**: Rapidly improving documentation and community
- **Better TypeScript Integration**: For mixed language projects

**Migration Impact**:
- ✅ **Performance**: Significantly faster reactive updates
- ✅ **Bundle Size**: Smaller WASM output due to fine-grained compilation
- ✅ **Developer Experience**: More intuitive reactive programming model
- ✅ **Cross-Platform**: Better Tauri integration for desktop apps
- ⚠️ **Migration Cost**: Required rewriting all frontend components
- ⚠️ **Learning Curve**: Team needs to learn Leptos patterns
- ✅ **Future-Proof**: Leptos is gaining momentum in Rust web development

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

## 2025-01-06: Cross-Platform Compilation Strategy

**Decision**: Configure build system for both web (WASM) and desktop (Tauri) targets

**Context**: Need to maximise market reach with single codebase

**Alternatives Considered**:
1. **Web-only**: Simpler but limits distribution options
2. **Progressive Web App**: Good mobile experience but limited desktop integration
3. **Separate native apps**: Much higher development cost
4. **Electron wrapper**: Larger bundle size and resource usage

**Rationale**:
- **Code Reuse**: 95%+ shared between platforms
- **Leptos Compatibility**: Excellent support for both targets
- **Market Coverage**: Web for accessibility, desktop for power users
- **Distribution Flexibility**: App stores + web deployment
- **Performance**: Native desktop performance when needed

**Impact**:
- ✅ **Broader Market**: Can serve both web and desktop users
- ✅ **Single Codebase**: Maintains development efficiency
- ✅ **Platform Optimisation**: Can tailor features per platform
- ⚠️ **Build Complexity**: Multiple compilation targets to maintain
- ⚠️ **Testing Overhead**: Need to test both platforms
- ✅ **Future-Proof**: Ready for mobile expansion via Tauri Mobile

---

## 2025-01-06: Reactive State Architecture with Leptos Signals

**Decision**: Replace Fermi atoms with Leptos built-in signal system

**Context**: Leptos provides more efficient and ergonomic state management than external libraries

**Alternatives Considered**:
1. **Keep Fermi**: Would work but misses Leptos benefits
2. **Redux Pattern**: Too complex for our needs
3. **Context API**: Less efficient than signals
4. **External Store**: Adds unnecessary dependency

**Rationale**:
- **Performance**: Fine-grained reactivity only updates changed values
- **Ergonomics**: More intuitive than global atoms
- **Integration**: Designed specifically for Leptos
- **Composition**: Easier to build derived state
- **Debugging**: Better development tools

**Impact**:
- ✅ **Better Performance**: Only affected components re-render
- ✅ **Cleaner Code**: Less boilerplate than Fermi atoms
- ✅ **Better DX**: Integrated error handling and debugging
- ⚠️ **Pattern Change**: Different mental model for state
- ✅ **Future-Ready**: Aligned with Leptos ecosystem direction

---

**Last Updated**: 2025-01-06 (Major Migration Completed)
**Next Review**: When evaluating backend framework or database decisions
**Migration Status**: ✅ Complete - All frontend components migrated to Leptos