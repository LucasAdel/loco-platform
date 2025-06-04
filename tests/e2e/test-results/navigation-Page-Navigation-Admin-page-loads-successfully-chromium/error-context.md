# Test info

- Name: Page Navigation >> Admin page loads successfully
- Location: /Users/hbl/Documents/loco-platform/tests/e2e/specs/navigation.spec.js:132:3

# Error details

```
Error: expect(received).toBeTruthy()

Received: false
    at /Users/hbl/Documents/loco-platform/tests/e2e/specs/navigation.spec.js:142:29
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
  - heading "Admin Panel" [level=1]
  - paragraph: Admin functionality coming soon...
```

# Test source

```ts
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
   77 |     expect(hasMapContainer || hasLoadingState).toBeTruthy();
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
> 142 |     expect(hasAdminContent).toBeTruthy();
      |                             ^ Error: expect(received).toBeTruthy()
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