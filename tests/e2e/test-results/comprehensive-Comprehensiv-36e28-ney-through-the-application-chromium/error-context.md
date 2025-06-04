# Test info

- Name: Comprehensive Application Tests >> Complete user journey through the application
- Location: /Users/hbl/Documents/loco-platform/tests/e2e/specs/comprehensive.spec.js:15:3

# Error details

```
Error: locator.click: Unexpected token "=" while parsing css selector "a[href="/jobs"], text="Jobs"". Did you mean to CSS.escape it?
Call log:
  - waiting for a[href="/jobs"], text="Jobs" >> nth=0

    at /Users/hbl/Documents/loco-platform/tests/e2e/specs/comprehensive.spec.js:24:66
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
  - heading "Welcome to Lo.Co Platform" [level=1]
  - paragraph: Your professional Australian pharmacy job marketplace built with Rust and Dioxus
  - text: 💼
  - heading "Job Marketplace" [level=3]
  - paragraph: Find pharmacy opportunities across Australia with advanced filtering and real-time updates.
  - text: 🗺️
  - heading "Interactive Maps" [level=3]
  - paragraph: Explore jobs on an interactive map with location-based search and commute calculations.
  - text: 🚀
  - heading "Built with Rust" [level=3]
  - paragraph: Experience blazing fast performance and memory safety with our Rust-powered platform.
  - text: 📱
  - heading "Progressive Web App" [level=3]
  - paragraph: Access your opportunities anywhere with our mobile-optimized PWA experience.
  - text: 🤖
  - heading "AI-Powered Matching" [level=3]
  - paragraph: Smart job recommendations based on your skills, location, and preferences.
  - text: 🔐
  - heading "Secure & Private" [level=3]
  - paragraph: Your data is protected with enterprise-grade security and privacy controls.
  - heading "Ready to get started?" [level=2]
  - paragraph: Explore available pharmacy positions or post new opportunities for your team.
  - button "Browse Jobs"
  - button "Post a Job"
```

# Test source

```ts
   1 | // Comprehensive integration tests using test utilities
   2 | const { test, expect } = require('@playwright/test');
   3 | const {
   4 |   waitForAppLoad,
   5 |   setupConsoleErrorTracking,
   6 |   navigateAndWait,
   7 |   elementExists,
   8 |   verifyPageLoaded,
   9 |   testResponsiveDesign,
   10 |   checkAccessibility,
   11 |   mockApiResponse
   12 | } = require('../utils/test-helpers');
   13 |
   14 | test.describe('Comprehensive Application Tests', () => {
   15 |   test('Complete user journey through the application', async ({ page }) => {
   16 |     const errors = setupConsoleErrorTracking(page);
   17 |     
   18 |     // Start at home page
   19 |     await navigateAndWait(page, '/');
   20 |     await verifyPageLoaded(page, '/');
   21 |     
   22 |     // Navigate to Jobs
   23 |     if (await elementExists(page, 'a[href="/jobs"], text="Jobs"')) {
>  24 |       await page.locator('a[href="/jobs"], text="Jobs"').first().click();
      |                                                                  ^ Error: locator.click: Unexpected token "=" while parsing css selector "a[href="/jobs"], text="Jobs"". Did you mean to CSS.escape it?
   25 |       await waitForAppLoad(page);
   26 |       await verifyPageLoaded(page, '/jobs');
   27 |     }
   28 |     
   29 |     // Navigate to Map
   30 |     if (await elementExists(page, 'a[href="/map"], text="Map"')) {
   31 |       await page.locator('a[href="/map"], text="Map"').first().click();
   32 |       await waitForAppLoad(page);
   33 |       await verifyPageLoaded(page, '/map');
   34 |     }
   35 |     
   36 |     // Navigate to Forum
   37 |     if (await elementExists(page, 'a[href="/forum"], text="Forum"')) {
   38 |       await page.locator('a[href="/forum"], text="Forum"').first().click();
   39 |       await waitForAppLoad(page);
   40 |       await verifyPageLoaded(page, '/forum');
   41 |     }
   42 |     
   43 |     // Return to home
   44 |     if (await elementExists(page, 'a[href="/"], text="Home"')) {
   45 |       await page.locator('a[href="/"], text="Home"').first().click();
   46 |       await waitForAppLoad(page);
   47 |       await verifyPageLoaded(page, '/');
   48 |     }
   49 |     
   50 |     // Check for critical errors
   51 |     const criticalErrors = errors.filter(error => 
   52 |       !error.includes('favicon') && 
   53 |       !error.includes('DevTools') &&
   54 |       !error.includes('Mapbox') && // Mapbox errors are expected without token
   55 |       !error.includes('token')
   56 |     );
   57 |     
   58 |     expect(criticalErrors).toHaveLength(0);
   59 |   });
   60 |
   61 |   test('Responsive design across all pages', async ({ page }) => {
   62 |     const pages = ['/', '/jobs', '/map', '/forum', '/profile'];
   63 |     
   64 |     for (const pagePath of pages) {
   65 |       await test.step(`Testing responsive design for ${pagePath}`, async () => {
   66 |         await testResponsiveDesign(page, async (page, viewport) => {
   67 |           await navigateAndWait(page, pagePath);
   68 |           
   69 |           // Basic checks for each viewport
   70 |           const bodyVisible = await page.locator('body').isVisible();
   71 |           expect(bodyVisible).toBeTruthy();
   72 |           
   73 |           // Check that content adapts to viewport
   74 |           const contentWidth = await page.locator('body').evaluate(el => el.scrollWidth);
   75 |           expect(contentWidth).toBeLessThanOrEqual(viewport.width + 50); // Small buffer for scrollbars
   76 |         });
   77 |       });
   78 |     }
   79 |   });
   80 |
   81 |   test('API integration with mock data', async ({ page }) => {
   82 |     // Mock jobs API
   83 |     await mockApiResponse(page, '/api/jobs', [
   84 |       {
   85 |         id: '1',
   86 |         title: 'Test Pharmacist Position',
   87 |         description: 'A test job for E2E testing',
   88 |         location: 'Sydney, NSW',
   89 |         type: 'Full-time'
   90 |       },
   91 |       {
   92 |         id: '2',
   93 |         title: 'Senior Pharmacy Manager',
   94 |         description: 'Management role in pharmacy',
   95 |         location: 'Melbourne, VIC', 
   96 |         type: 'Full-time'
   97 |       }
   98 |     ]);
   99 |     
  100 |     await navigateAndWait(page, '/jobs');
  101 |     
  102 |     // Wait for jobs to load
  103 |     await page.waitForTimeout(3000);
  104 |     
  105 |     // Should show mocked jobs
  106 |     const jobElements = await page.locator('[data-testid="job-card"], .job-card, .job-item').count();
  107 |     const hasJobContent = await elementExists(page, 'text="Test Pharmacist Position"');
  108 |     
  109 |     expect(jobElements > 0 || hasJobContent).toBeTruthy();
  110 |   });
  111 |
  112 |   test('Error handling across the application', async ({ page }) => {
  113 |     const errors = setupConsoleErrorTracking(page);
  114 |     
  115 |     // Test 404 page
  116 |     await page.goto('/non-existent-page');
  117 |     await waitForAppLoad(page);
  118 |     
  119 |     // Should handle 404 gracefully
  120 |     const has404Content = await elementExists(page, 'text=/404|not found|page not found/i');
  121 |     const stillHasNavigation = await elementExists(page, 'nav, [role="navigation"]');
  122 |     
  123 |     expect(has404Content || stillHasNavigation).toBeTruthy();
  124 |     
```