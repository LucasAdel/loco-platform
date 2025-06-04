# Test info

- Name: Page Navigation >> Map page loads successfully
- Location: /Users/hbl/Documents/loco-platform/tests/e2e/specs/navigation.spec.js:62:3

# Error details

```
Error: expect(received).toBeTruthy()

Received: false
    at /Users/hbl/Documents/loco-platform/tests/e2e/specs/navigation.spec.js:77:48
```

# Page snapshot

```yaml
- text: Lo.Co Platform Australian Pharmacy Jobs
- navigation:
  - list:
    - listitem:
      - button "🏠 Home"
    - listitem:
      - button "💼 Jobs"
    - listitem:
      - button "🗺️ Map"
    - listitem:
      - button "💬 Forum"
    - listitem:
      - button "👤 Profile"
    - listitem:
      - button "🔔 Notifications"
    - listitem:
      - button "📅 Availability"
    - listitem:
      - button "⚙️ Admin Panel"
    - listitem:
      - button "🔗 Lo.Co Connect"
- main:
  - text: 5 jobs 3 2
  - textbox "Search for address, suburb or postcode..."
  - img
  - text: 5 jobs found
  - checkbox "🚨 Urgent only"
  - text: 🚨 Urgent only
  - heading "Job Types" [level=4]
  - checkbox "💊 Pharmacist" [checked]
  - text: 💊 Pharmacist
  - checkbox "📚 Intern" [checked]
  - text: 📚 Intern
  - checkbox "🎓 Student" [checked]
  - text: 🎓 Student
  - checkbox "🩺 Assistant" [checked]
  - text: 🩺 Assistant
  - button "+"
  - button "−"
  - button "🌞"
  - button "🎯"
  - button "🗺️"
  - text: "Sort by:"
  - combobox:
    - option "Distance" [selected]
    - option "Date"
    - option "Rate"
  - button "📋"
  - heading "Available Positions" [level=3]
  - paragraph: 5 jobs in Adelaide
  - text: Pharmacist
  - heading "Pharmacist Position 3" [level=4]
  - paragraph: Pharmacy C
  - text: 📍 Adelaide, SA 🚶 7.1km $50/hr
  - button "👁"
  - button "💾"
  - text: Student URGENT
  - heading "Student Position 5" [level=4]
  - paragraph: Pharmacy E
  - text: 📍 Adelaide, SA 🚶 10.9km $42/hr
  - button "👁"
  - button "💾"
  - text: Student
  - heading "Student Position 4" [level=4]
  - paragraph: Pharmacy D
  - text: 📍 Adelaide, SA 🚶 11.3km $35/hr
  - button "👁"
  - button "💾"
  - text: Intern
  - heading "Intern Position 2" [level=4]
  - paragraph: Pharmacy B
  - text: 📍 Adelaide, SA 🚶 12.4km $41/hr
  - button "👁"
  - button "💾"
  - text: Intern
  - heading "Intern Position 1" [level=4]
  - paragraph: Pharmacy A
  - text: 📍 Adelaide, SA 🚶 16.1km $39/hr
  - button "👁"
  - button "💾"
  - text: © Mapbox © OpenStreetMap
  - link "Improve this map":
    - /url: "#"
```

# Test source

```ts
   1 | // Navigation and page loading tests for Loco Platform
   2 | const { test, expect } = require('@playwright/test');
   3 |
   4 | test.describe('Page Navigation', () => {
   5 |   test.beforeEach(async ({ page }) => {
   6 |     // Listen for console errors
   7 |     page.on('console', (msg) => {
   8 |       if (msg.type() === 'error') {
   9 |         console.error(`Console error: ${msg.text()}`);
   10 |       }
   11 |     });
   12 |
   13 |     // Listen for uncaught exceptions
   14 |     page.on('pageerror', (exception) => {
   15 |       console.error(`Uncaught exception: ${exception}`);
   16 |     });
   17 |   });
   18 |
   19 |   test('Home page loads successfully', async ({ page }) => {
   20 |     await page.goto('/');
   21 |     
   22 |     // Wait for the page to load
   23 |     await page.waitForLoadState('networkidle');
   24 |     
   25 |     // Check that the page title is present
   26 |     await expect(page).toHaveTitle(/Loco Platform/);
   27 |     
   28 |     // Verify no console errors (check console messages)
   29 |     const errors = [];
   30 |     page.on('console', msg => {
   31 |       if (msg.type() === 'error') {
   32 |         errors.push(msg.text());
   33 |       }
   34 |     });
   35 |     
   36 |     // Wait a bit to catch any async errors
   37 |     await page.waitForTimeout(2000);
   38 |     
   39 |     // Should have no critical errors
   40 |     expect(errors.filter(error => 
   41 |       !error.includes('favicon') && 
   42 |       !error.includes('DevTools')
   43 |     )).toHaveLength(0);
   44 |   });
   45 |
   46 |   test('Jobs page loads successfully', async ({ page }) => {
   47 |     await page.goto('/jobs');
   48 |     
   49 |     await page.waitForLoadState('networkidle');
   50 |     
   51 |     // Should be on jobs page
   52 |     expect(page.url()).toContain('/jobs');
   53 |     
   54 |     // Check for jobs content or loading state
   55 |     const hasJobsContent = await page.locator('[data-testid="jobs-container"], .jobs-container, .job-list').count() > 0;
   56 |     const hasLoadingState = await page.locator('[data-testid="loading"], .loading').count() > 0;
   57 |     
   58 |     // Should have either jobs content or loading state
   59 |     expect(hasJobsContent || hasLoadingState).toBeTruthy();
   60 |   });
   61 |
   62 |   test('Map page loads successfully', async ({ page }) => {
   63 |     await page.goto('/map');
   64 |     
   65 |     await page.waitForLoadState('networkidle');
   66 |     
   67 |     // Should be on map page
   68 |     expect(page.url()).toContain('/map');
   69 |     
   70 |     // Give Mapbox time to load
   71 |     await page.waitForTimeout(3000);
   72 |     
   73 |     // Check for map container or loading state
   74 |     const hasMapContainer = await page.locator('[data-testid="map-container"], .map-container, #map').count() > 0;
   75 |     const hasLoadingState = await page.locator('[data-testid="loading"], .loading').count() > 0;
   76 |     
>  77 |     expect(hasMapContainer || hasLoadingState).toBeTruthy();
      |                                                ^ Error: expect(received).toBeTruthy()
   78 |   });
   79 |
   80 |   test('Forum page loads successfully', async ({ page }) => {
   81 |     await page.goto('/forum');
   82 |     
   83 |     await page.waitForLoadState('networkidle');
   84 |     
   85 |     // Should be on forum page
   86 |     expect(page.url()).toContain('/forum');
   87 |     
   88 |     // Check for forum content (even if placeholder)
   89 |     const hasForumContent = await page.locator('text=/forum/i, text=/discussion/i, text=/coming soon/i').count() > 0;
   90 |     expect(hasForumContent).toBeTruthy();
   91 |   });
   92 |
   93 |   test('Profile page loads successfully', async ({ page }) => {
   94 |     await page.goto('/profile');
   95 |     
   96 |     await page.waitForLoadState('networkidle');
   97 |     
   98 |     // Should be on profile page
   99 |     expect(page.url()).toContain('/profile');
  100 |     
  101 |     // Check for profile content (even if placeholder)
  102 |     const hasProfileContent = await page.locator('text=/profile/i, text=/account/i, text=/coming soon/i').count() > 0;
  103 |     expect(hasProfileContent).toBeTruthy();
  104 |   });
  105 |
  106 |   test('Notifications page loads successfully', async ({ page }) => {
  107 |     await page.goto('/notifications');
  108 |     
  109 |     await page.waitForLoadState('networkidle');
  110 |     
  111 |     // Should be on notifications page
  112 |     expect(page.url()).toContain('/notifications');
  113 |     
  114 |     // Check for notifications content (even if placeholder)
  115 |     const hasNotificationsContent = await page.locator('text=/notification/i, text=/alert/i, text=/coming soon/i').count() > 0;
  116 |     expect(hasNotificationsContent).toBeTruthy();
  117 |   });
  118 |
  119 |   test('Availability page loads successfully', async ({ page }) => {
  120 |     await page.goto('/availability');
  121 |     
  122 |     await page.waitForLoadState('networkidle');
  123 |     
  124 |     // Should be on availability page
  125 |     expect(page.url()).toContain('/availability');
  126 |     
  127 |     // Check for availability content (even if placeholder)
  128 |     const hasAvailabilityContent = await page.locator('text=/availability/i, text=/schedule/i, text=/coming soon/i').count() > 0;
  129 |     expect(hasAvailabilityContent).toBeTruthy();
  130 |   });
  131 |
  132 |   test('Admin page loads successfully', async ({ page }) => {
  133 |     await page.goto('/admin');
  134 |     
  135 |     await page.waitForLoadState('networkidle');
  136 |     
  137 |     // Should be on admin page
  138 |     expect(page.url()).toContain('/admin');
  139 |     
  140 |     // Check for admin content (even if placeholder)
  141 |     const hasAdminContent = await page.locator('text=/admin/i, text=/dashboard/i, text=/coming soon/i').count() > 0;
  142 |     expect(hasAdminContent).toBeTruthy();
  143 |   });
  144 |
  145 |   test('Connect page loads successfully', async ({ page }) => {
  146 |     await page.goto('/connect');
  147 |     
  148 |     await page.waitForLoadState('networkidle');
  149 |     
  150 |     // Should be on connect page
  151 |     expect(page.url()).toContain('/connect');
  152 |     
  153 |     // Check for connect content (even if placeholder)
  154 |     const hasConnectContent = await page.locator('text=/connect/i, text=/network/i, text=/coming soon/i').count() > 0;
  155 |     expect(hasConnectContent).toBeTruthy();
  156 |   });
  157 | });
```