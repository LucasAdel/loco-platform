# Test info

- Name: Map Page >> Map page renders correctly
- Location: /Users/hbl/Documents/loco-platform/tests/e2e/specs/map.spec.js:10:3

# Error details

```
Error: expect(received).toBeTruthy()

Received: false
    at /Users/hbl/Documents/loco-platform/tests/e2e/specs/map.spec.js:23:48
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
   1 | // Map page functionality tests
   2 | const { test, expect } = require('@playwright/test');
   3 |
   4 | test.describe('Map Page', () => {
   5 |   test.beforeEach(async ({ page }) => {
   6 |     await page.goto('/map');
   7 |     await page.waitForLoadState('networkidle');
   8 |   });
   9 |
   10 |   test('Map page renders correctly', async ({ page }) => {
   11 |     // Check if we're on the map page
   12 |     expect(page.url()).toContain('/map');
   13 |     
   14 |     // Look for map container
   15 |     const mapContainer = page.locator('#map, [data-testid="map"], .map-container, .mapbox-map');
   16 |     
   17 |     // Give Mapbox time to load
   18 |     await page.waitForTimeout(5000);
   19 |     
   20 |     const hasMapContainer = await mapContainer.count() > 0;
   21 |     const hasLoadingState = await page.locator('[data-testid="loading"], .loading, .map-loading').count() > 0;
   22 |     
>  23 |     expect(hasMapContainer || hasLoadingState).toBeTruthy();
      |                                                ^ Error: expect(received).toBeTruthy()
   24 |   });
   25 |
   26 |   test('Mapbox integration loads', async ({ page }) => {
   27 |     // Wait for Mapbox to potentially load
   28 |     await page.waitForTimeout(10000);
   29 |     
   30 |     // Check for Mapbox-specific elements
   31 |     const mapboxElements = await page.locator('.mapboxgl-map, .mapbox-gl-map, [class*="mapbox"]').count() > 0;
   32 |     const canvasElement = await page.locator('canvas').count() > 0;
   33 |     
   34 |     // Check for map-related JavaScript errors
   35 |     const errors = [];
   36 |     page.on('console', msg => {
   37 |       if (msg.type() === 'error' && !msg.text().includes('favicon')) {
   38 |         errors.push(msg.text());
   39 |       }
   40 |     });
   41 |     
   42 |     // Mapbox should either load or show a meaningful error
   43 |     const hasMapError = errors.some(error => 
   44 |       error.includes('map') || 
   45 |       error.includes('Mapbox') || 
   46 |       error.includes('token')
   47 |     );
   48 |     
   49 |     // Should have map elements or clear error about missing token
   50 |     if (hasMapError) {
   51 |       console.log('Map errors (expected if no Mapbox token):', errors);
   52 |     }
   53 |     
   54 |     expect(mapboxElements || canvasElement || hasMapError).toBeTruthy();
   55 |   });
   56 |
   57 |   test('Map controls are present', async ({ page }) => {
   58 |     await page.waitForTimeout(5000);
   59 |     
   60 |     // Look for map controls
   61 |     const zoomControls = await page.locator('.mapboxgl-ctrl-zoom, .map-zoom, [data-testid="zoom-controls"]').count() > 0;
   62 |     const navigationControls = await page.locator('.mapboxgl-ctrl, .map-controls').count() > 0;
   63 |     
   64 |     // If map loads, should have some controls
   65 |     const mapLoaded = await page.locator('.mapboxgl-map, canvas').count() > 0;
   66 |     
   67 |     if (mapLoaded) {
   68 |       expect(zoomControls || navigationControls).toBeTruthy();
   69 |     }
   70 |   });
   71 |
   72 |   test('Job markers functionality', async ({ page }) => {
   73 |     await page.waitForTimeout(8000);
   74 |     
   75 |     // Look for job markers on the map
   76 |     const markers = page.locator('.mapboxgl-marker, .marker, [data-testid="job-marker"]');
   77 |     const markerCount = await markers.count();
   78 |     
   79 |     if (markerCount > 0) {
   80 |       // Test marker interaction
   81 |       const firstMarker = markers.first();
   82 |       await firstMarker.click();
   83 |       
   84 |       await page.waitForTimeout(1000);
   85 |       
   86 |       // Should show popup or job details
   87 |       const popup = await page.locator('.mapboxgl-popup, .map-popup, [data-testid="job-popup"]').count() > 0;
   88 |       const jobDetails = await page.locator('.job-details, [data-testid="job-details"]').count() > 0;
   89 |       
   90 |       expect(popup || jobDetails).toBeTruthy();
   91 |     } else {
   92 |       // If no markers, that's also valid (no jobs to show)
   93 |       console.log('No job markers found on map');
   94 |     }
   95 |   });
   96 |
   97 |   test('Map responds to user interaction', async ({ page }) => {
   98 |     await page.waitForTimeout(5000);
   99 |     
  100 |     const mapContainer = page.locator('#map, [data-testid="map"], .map-container').first();
  101 |     
  102 |     if (await mapContainer.count() > 0) {
  103 |       // Test map interaction (pan/zoom)
  104 |       const boundingBox = await mapContainer.boundingBox();
  105 |       
  106 |       if (boundingBox) {
  107 |         // Simulate map pan
  108 |         await page.mouse.move(boundingBox.x + boundingBox.width / 2, boundingBox.y + boundingBox.height / 2);
  109 |         await page.mouse.down();
  110 |         await page.mouse.move(boundingBox.x + boundingBox.width / 2 + 50, boundingBox.y + boundingBox.height / 2 + 50);
  111 |         await page.mouse.up();
  112 |         
  113 |         await page.waitForTimeout(1000);
  114 |         
  115 |         // Map should still be functional after interaction
  116 |         const mapStillExists = await mapContainer.count() > 0;
  117 |         expect(mapStillExists).toBeTruthy();
  118 |       }
  119 |     }
  120 |   });
  121 |
  122 |   test('Location search functionality', async ({ page }) => {
  123 |     // Look for location search input
```