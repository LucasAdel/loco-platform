# Test info

- Name: Page Navigation >> Profile page loads successfully
- Location: /Users/hbl/Documents/loco-platform/tests/e2e/specs/navigation.spec.js:96:3

# Error details

```
Error: page.goto: net::ERR_SOCKET_NOT_CONNECTED at http://localhost:8080/profile
Call log:
  - navigating to "http://localhost:8080/profile", waiting until "load"

    at /Users/hbl/Documents/loco-platform/tests/e2e/specs/navigation.spec.js:97:16
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
   77 |     expect(hasMapContainer || hasLoadingState).toBeTruthy();
   78 |   });
   79 |
   80 |   test('Forum page loads successfully', async ({ page }) => {
   81 |     await page.goto('/forum');
   82 |     
   83 |     await page.waitForLoadState('networkidle');
   84 |     await page.waitForTimeout(1000); // Give JS routing time
   85 |     
   86 |     // Should be on forum page
   87 |     expect(page.url()).toContain('/forum');
   88 |     
   89 |     // Check for specific forum content
   90 |     const hasForumHeading = await page.locator('h2:has-text("Professional Forum")').count() > 0;
   91 |     const hasComingSoon = await page.locator('text=Forum features coming soon').count() > 0;
   92 |     
   93 |     expect(hasForumHeading || hasComingSoon).toBeTruthy();
   94 |   });
   95 |
   96 |   test('Profile page loads successfully', async ({ page }) => {
>  97 |     await page.goto('/profile');
      |                ^ Error: page.goto: net::ERR_SOCKET_NOT_CONNECTED at http://localhost:8080/profile
   98 |     
   99 |     await page.waitForLoadState('networkidle');
  100 |     await page.waitForTimeout(1000); // Give JS routing time
  101 |     
  102 |     // Should be on profile page
  103 |     expect(page.url()).toContain('/profile');
  104 |     
  105 |     // Check for specific profile content
  106 |     const hasProfileHeading = await page.locator('h2:has-text("Your Profile")').count() > 0;
  107 |     const hasComingSoon = await page.locator('text=Profile management coming soon').count() > 0;
  108 |     
  109 |     expect(hasProfileHeading || hasComingSoon).toBeTruthy();
  110 |   });
  111 |
  112 |   test('Notifications page loads successfully', async ({ page }) => {
  113 |     await page.goto('/notifications');
  114 |     
  115 |     await page.waitForLoadState('networkidle');
  116 |     await page.waitForTimeout(1000); // Give JS routing time
  117 |     
  118 |     // Should be on notifications page
  119 |     expect(page.url()).toContain('/notifications');
  120 |     
  121 |     // Check for specific notifications content
  122 |     const hasNotificationsHeading = await page.locator('h2:has-text("Notifications")').count() > 0;
  123 |     const hasComingSoon = await page.locator('text=Notification system coming soon').count() > 0;
  124 |     
  125 |     expect(hasNotificationsHeading || hasComingSoon).toBeTruthy();
  126 |   });
  127 |
  128 |   test('Availability page loads successfully', async ({ page }) => {
  129 |     await page.goto('/availability');
  130 |     
  131 |     await page.waitForLoadState('networkidle');
  132 |     await page.waitForTimeout(1000); // Give JS routing time
  133 |     
  134 |     // Should be on availability page
  135 |     expect(page.url()).toContain('/availability');
  136 |     
  137 |     // Check for specific availability content
  138 |     const hasAvailabilityHeading = await page.locator('h2:has-text("Availability Management")').count() > 0;
  139 |     const hasComingSoon = await page.locator('text=Availability features coming soon').count() > 0;
  140 |     
  141 |     expect(hasAvailabilityHeading || hasComingSoon).toBeTruthy();
  142 |   });
  143 |
  144 |   test('Admin page loads successfully', async ({ page }) => {
  145 |     await page.goto('/admin');
  146 |     
  147 |     await page.waitForLoadState('networkidle');
  148 |     await page.waitForTimeout(1000); // Give JS routing time
  149 |     
  150 |     // Should be on admin page
  151 |     expect(page.url()).toContain('/admin');
  152 |     
  153 |     // Check for specific admin content
  154 |     const hasAdminHeading = await page.locator('h2:has-text("Admin Dashboard")').count() > 0;
  155 |     const hasComingSoon = await page.locator('text=Admin features coming soon').count() > 0;
  156 |     
  157 |     expect(hasAdminHeading || hasComingSoon).toBeTruthy();
  158 |   });
  159 |
  160 |   test('Connect page loads successfully', async ({ page }) => {
  161 |     await page.goto('/connect');
  162 |     
  163 |     await page.waitForLoadState('networkidle');
  164 |     await page.waitForTimeout(1000); // Give JS routing time
  165 |     
  166 |     // Should be on connect page
  167 |     expect(page.url()).toContain('/connect');
  168 |     
  169 |     // Check for specific connect content
  170 |     const hasConnectHeading = await page.locator('h2:has-text("Lo.Co Connect")').count() > 0;
  171 |     const hasComingSoon = await page.locator('text=Connect features coming soon').count() > 0;
  172 |     
  173 |     expect(hasConnectHeading || hasComingSoon).toBeTruthy();
  174 |   });
  175 | });
```