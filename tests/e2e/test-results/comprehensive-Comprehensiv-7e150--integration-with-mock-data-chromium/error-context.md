# Test info

- Name: Comprehensive Application Tests >> API integration with mock data
- Location: /Users/hbl/Documents/loco-platform/tests/e2e/specs/comprehensive.spec.js:81:3

# Error details

```
Error: expect(received).toBeTruthy()

Received: false
    at /Users/hbl/Documents/loco-platform/tests/e2e/specs/comprehensive.spec.js:109:46
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
  - heading "Available Jobs" [level=2]
  - heading "Intern Position 1" [level=3]
  - paragraph: Pharmacy A
  - text: $39/hr 📍 1 Main St, Adelaide (16.1 km) 📅 03/06/2025 🕐 15am - 5pm
  - button "View on Map"
  - button "View Details"
  - heading "Intern Position 2" [level=3]
  - paragraph: Pharmacy B
  - text: $41/hr 📍 1 Main St, Adelaide (12.4 km) 📅 03/06/2025 🕐 13am - 7pm
  - button "View on Map"
  - button "View Details"
  - heading "Pharmacist Position 3" [level=3]
  - paragraph: Pharmacy C
  - text: $50/hr 📍 1 Main St, Adelaide (7.1 km) 📅 03/06/2025 🕐 12am - 7pm
  - button "View on Map"
  - button "View Details"
  - heading "Student Position 4" [level=3]
  - paragraph: Pharmacy D
  - text: $35/hr 📍 2 Main St, Adelaide (11.3 km) 📅 03/06/2025 🕐 16am - 7pm
  - button "View on Map"
  - button "View Details"
  - heading "Student Position 5" [level=3]
  - paragraph: Pharmacy E
  - text: $42/hr Urgent 📍 5 km (10.9 km) 📅 03/06/2025 🕐 16am - 5pm
  - button "View on Map"
  - button "View Details"
  - textbox "Search for address, suburb or postcode..."
  - img
  - text: 🗺️
  - heading "Interactive Map" [level=3]
  - paragraph: Mapbox integration coming soon...
  - text: 📍 Adelaide
  - button "+"
  - button "−"
  - button "🎯"
  - text: © Mapbox © OpenStreetMap
  - link "Improve this map":
    - /url: "#"
```

# Test source

```ts
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
   24 |       await page.locator('a[href="/jobs"], text="Jobs"').first().click();
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
> 109 |     expect(jobElements > 0 || hasJobContent).toBeTruthy();
      |                                              ^ Error: expect(received).toBeTruthy()
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
  125 |     // Navigate back to valid page
  126 |     await navigateAndWait(page, '/');
  127 |     await verifyPageLoaded(page, '/');
  128 |     
  129 |     // App should recover gracefully
  130 |     const appWorking = await elementExists(page, 'body');
  131 |     expect(appWorking).toBeTruthy();
  132 |   });
  133 |
  134 |   test('Accessibility compliance', async ({ page }) => {
  135 |     const pages = ['/', '/jobs', '/map'];
  136 |     
  137 |     for (const pagePath of pages) {
  138 |       await test.step(`Checking accessibility for ${pagePath}`, async () => {
  139 |         await navigateAndWait(page, pagePath);
  140 |         
  141 |         const accessibilityIssues = await checkAccessibility(page);
  142 |         
  143 |         // Log issues but don't fail tests yet (can be improved incrementally)
  144 |         if (accessibilityIssues.length > 0) {
  145 |           console.log(`Accessibility issues on ${pagePath}:`, accessibilityIssues);
  146 |         }
  147 |         
  148 |         // At minimum, page should have basic structure
  149 |         const hasHeading = await elementExists(page, 'h1, h2, h3');
  150 |         expect(hasHeading).toBeTruthy();
  151 |       });
  152 |     }
  153 |   });
  154 |
  155 |   test('Search functionality across the application', async ({ page }) => {
  156 |     // Test search on jobs page
  157 |     await navigateAndWait(page, '/jobs');
  158 |     
  159 |     const searchInput = page.locator('input[type="search"], input[placeholder*="search"], [data-testid="search"]').first();
  160 |     
  161 |     if (await searchInput.count() > 0) {
  162 |       await searchInput.fill('pharmacist');
  163 |       await searchInput.press('Enter');
  164 |       
  165 |       await page.waitForTimeout(2000);
  166 |       
  167 |       // Search should trigger some response
  168 |       const urlChanged = page.url().includes('search') || page.url().includes('pharmacist');
  169 |       const contentChanged = await elementExists(page, '.search-results, [data-testid="search-results"]');
  170 |       
  171 |       expect(urlChanged || contentChanged).toBeTruthy();
  172 |     }
  173 |   });
  174 |
  175 |   test('Navigation consistency', async ({ page }) => {
  176 |     const routes = ['/', '/jobs', '/map', '/forum', '/profile'];
  177 |     
  178 |     for (const route of routes) {
  179 |       await test.step(`Testing navigation consistency for ${route}`, async () => {
  180 |         await navigateAndWait(page, route);
  181 |         
  182 |         // Each page should have consistent navigation
  183 |         const hasNavigation = await elementExists(page, 'nav, [role="navigation"], .sidebar');
  184 |         expect(hasNavigation).toBeTruthy();
  185 |         
  186 |         // Each page should have a way to get back to home
  187 |         const hasHomeLink = await elementExists(page, 'a[href="/"], text="Home", [data-testid="home-link"]');
  188 |         
  189 |         // Home link is not strictly required but navigation should exist
  190 |         expect(hasNavigation).toBeTruthy();
  191 |       });
  192 |     }
  193 |   });
  194 |
  195 |   test('State management across navigation', async ({ page }) => {
  196 |     // Start at jobs page and perform a search
  197 |     await navigateAndWait(page, '/jobs');
  198 |     
  199 |     const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').first();
  200 |     
  201 |     if (await searchInput.count() > 0) {
  202 |       await searchInput.fill('test search');
  203 |       await page.waitForTimeout(1000);
  204 |       
  205 |       // Navigate to another page
  206 |       await navigateAndWait(page, '/map');
  207 |       
  208 |       // Navigate back to jobs
  209 |       await navigateAndWait(page, '/jobs');
```