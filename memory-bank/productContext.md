# Product Context - Loco Platform

## 🏗️ System Architecture

### High-Level Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                         Frontend (Dioxus)                     │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────────┐  │
│  │   Pages     │  │  Components  │  │  State (Fermi)    │  │
│  │  - Jobs     │  │  - JobCard   │  │  - User State     │  │
│  │  - Map      │  │  - SearchBar │  │  - Job Filters    │  │
│  │  - Profile  │  │  - Sidebar   │  │  - UI Theme       │  │
│  └─────────────┘  └──────────────┘  └───────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ HTTP/WebSocket
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Backend (Axum)                          │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────────┐  │
│  │  Handlers   │  │   Services   │  │   Middleware      │  │
│  │  - Jobs     │  │  - JobSvc    │  │  - Auth (JWT)     │  │
│  │  - Users    │  │  - UserSvc   │  │  - CORS           │  │
│  │  - Auth     │  │  - AuthSvc   │  │  - Logging        │  │
│  └─────────────┘  └──────────────┘  └───────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ SeaORM
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Data Layer                                │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────────┐  │
│  │ PostgreSQL  │  │    Redis     │  │   File Storage    │  │
│  │  - Users    │  │  - Sessions  │  │  - Avatars        │  │
│  │  - Jobs     │  │  - Cache     │  │  - Documents      │  │
│  │  - Apps     │  │  - Queues    │  │  - Attachments    │  │
│  └─────────────┘  └──────────────┘  └───────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Component Hierarchy

#### Frontend Components
```
App
├── Router
│   ├── Layout
│   │   ├── Sidebar (Navigation)
│   │   └── MainContent
│   │       ├── HomePage
│   │       ├── JobsPage
│   │       │   ├── SearchBar
│   │       │   ├── FilterPanel
│   │       │   └── JobList
│   │       │       └── JobCard
│   │       ├── MapPage
│   │       │   ├── MapView
│   │       │   ├── JobMarkers
│   │       │   └── LocationControls
│   │       ├── ProfilePage
│   │       └── AdminPage
│   └── AuthGuard
└── GlobalProviders
    ├── ThemeProvider
    ├── AuthProvider
    └── StateProvider
```

## 🎨 Design System

### Colour Palette
```rust
// Australian-inspired colour scheme
pub struct ColourPalette {
    // Primary colours
    pub eucalyptus_green: "#2D5A3D",     // Primary brand
    pub ocean_blue: "#006994",            // Secondary
    pub sunset_gold: "#FFB81C",           // Accent
    
    // Neutral colours  
    pub charcoal: "#2C2C2C",              // Text primary
    pub slate: "#64748B",                 // Text secondary
    pub pearl: "#F8FAFC",                 // Background
    
    // Semantic colours
    pub success: "#10B981",               // Green
    pub warning: "#F59E0B",               // Amber
    pub error: "#EF4444",                 // Red
    pub info: "#3B82F6",                  // Blue
}
```

### Typography
```css
/* Font hierarchy */
--font-display: 'Inter', sans-serif;     /* Headings */
--font-body: 'Inter', sans-serif;        /* Body text */
--font-mono: 'JetBrains Mono', monospace; /* Code */

/* Size scale */
--text-xs: 0.75rem;    /* 12px */
--text-sm: 0.875rem;   /* 14px */
--text-base: 1rem;     /* 16px */
--text-lg: 1.125rem;   /* 18px */
--text-xl: 1.25rem;    /* 20px */
--text-2xl: 1.5rem;    /* 24px */
--text-3xl: 1.875rem;  /* 30px */
```

### Component Styling
- **Glass Morphism**: Modern translucent design with backdrop blur
- **Smooth Animations**: 200-300ms transitions for interactions
- **Consistent Spacing**: 8px grid system (0.5rem increments)
- **Responsive Breakpoints**: Mobile-first approach
  - Mobile: 0-640px
  - Tablet: 641-1024px
  - Desktop: 1025px+

## 🔄 Data Flow

### State Management Architecture
```rust
// Global state atoms (Fermi)
static CURRENT_USER: Atom<Option<User>> = Atom(|_| None);
static JOB_FILTERS: Atom<JobFilters> = Atom(|_| JobFilters::default());
static JOB_LIST: Atom<Vec<Job>> = Atom(|_| vec![]);
static UI_THEME: Atom<Theme> = Atom(|_| Theme::Light);
```

### API Communication Flow
1. **User Action** → Component event handler
2. **API Request** → ApiClient service method
3. **Backend Processing** → Axum handler → Service → Repository
4. **Response** → State update → UI re-render

### Real-time Updates (WebSocket)
```
Client connects → WS handshake → Subscribe to channels
Server event → Broadcast to subscribers → Client state update
```

## 🗺️ Feature Map

### Current Features (Implemented)
1. **Foundation**
   - ✅ Rust workspace structure
   - ✅ Shared type system
   - ✅ Basic routing
   - ✅ Component architecture

2. **UI Components**
   - ✅ Responsive sidebar
   - ✅ Job card design
   - ✅ Search bar
   - ✅ Theme system

3. **Backend Structure**
   - ✅ Axum server setup
   - ✅ Handler architecture
   - ✅ Service layer
   - ✅ Middleware stack

4. **Map Features** (December 2024)
   - ✅ Comprehensive map page
   - ✅ Apple-style glass morphism UI
   - ✅ Location management with GPS
   - ✅ Advanced filtering system
   - ✅ Real-time job updates
   - ✅ Distance calculations

### Planned Features (Priority Order)

#### Phase 1: Core Functionality
- [ ] User authentication (JWT)
- [ ] Job CRUD operations
- [ ] Application workflow
- [ ] Basic messaging

#### Phase 2: Enhanced Features
- [ ] Advanced search with filters
- [ ] Email notifications
- [ ] File uploads
- [ ] Calendar integration

#### Phase 3: Advanced Capabilities
- [ ] AI job recommendations
- [ ] Video interviews
- [ ] Analytics dashboard
- [ ] Mobile app

## 📱 User Journeys

### Professional Journey
```
1. Land on homepage → See value proposition
2. Browse jobs → Filter by location/type/salary
3. View job details → Check requirements
4. Apply with profile → Upload documents
5. Track applications → Receive updates
6. Schedule interview → Get hired
```

### Employer Journey
```
1. Register business → Verify credentials
2. Post job opening → Use templates
3. Review applications → Filter candidates
4. Message applicants → Schedule interviews
5. Make offer → Track acceptance
6. Manage team → View analytics
```

## 🔧 Technical Decisions

### Why Dioxus?
- **Performance**: Compiles to WASM for near-native speed
- **Type Safety**: Catches errors at compile time
- **Familiar Syntax**: React-like components
- **Cross-platform**: Web, desktop, mobile from single codebase

### Why Axum?
- **Modern**: Built on Tokio async runtime
- **Type Safe**: Leverages Rust's type system
- **Performant**: Efficient request handling
- **Ecosystem**: Excellent middleware support

### Why SeaORM?
- **Async First**: Built for async Rust
- **Type Safe**: Compile-time query validation
- **Migration System**: Version control for schema
- **Dynamic Queries**: Flexible query building

## 🚀 Performance Targets

### Frontend Metrics
- **First Paint**: < 1 second
- **Interactive**: < 2 seconds
- **Bundle Size**: < 500KB gzipped
- **Memory Usage**: < 50MB active

### Backend Metrics
- **API Latency**: p95 < 100ms
- **Throughput**: 10,000 req/sec
- **Database Queries**: < 50ms
- **WebSocket Connections**: 50,000 concurrent

## 🔐 Security Model

### Authentication Flow
1. User submits credentials
2. Server validates against database
3. Generate JWT with claims
4. Client stores token securely
5. Include token in API requests
6. Server validates on each request

### Authorisation Levels
- **Guest**: Browse public jobs
- **Professional**: Apply for jobs, manage profile
- **Employer**: Post jobs, manage applications
- **Admin**: Full system access

### Data Protection
- **Encryption**: TLS 1.3 for transit, AES-256 at rest
- **Passwords**: Argon2 hashing with salt
- **Sessions**: Redis with TTL expiration
- **PII**: Encrypted and access-controlled

## 📈 Analytics & Monitoring

### User Analytics
- Page views and navigation flow
- Job search patterns
- Application conversion rates
- User engagement metrics

### System Monitoring
- Server health and uptime
- API endpoint performance
- Database query performance
- Error rates and logging

### Business Metrics
- Active users (DAU/MAU)
- Job posting volume
- Application success rate
- Revenue tracking

## 🌐 Deployment Strategy

### Development Environment
```bash
# Frontend
dx serve --hot-reload

# Backend  
cargo watch -x run

# Database
docker-compose up postgres redis
```

### Production Environment
- **Frontend**: CDN-distributed static files
- **Backend**: Kubernetes pods with auto-scaling
- **Database**: Managed PostgreSQL with replicas
- **Cache**: Redis cluster for high availability

## 🔄 Continuous Improvement

### Feedback Loops
1. User feedback → Feature requests
2. Analytics data → UX improvements
3. Performance monitoring → Optimisations
4. Security audits → Vulnerability fixes

### Update Cadence
- **Hotfixes**: As needed for critical issues
- **Features**: Bi-weekly releases
- **Major Updates**: Quarterly planning
- **Security**: Monthly dependency updates

---

**Last Updated**: January 2025
**Version**: 2.0
**Next Review**: February 2025