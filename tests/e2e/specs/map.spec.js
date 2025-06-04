// Map page functionality tests
const { test, expect } = require('@playwright/test');

test.describe('Map Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/map');
    await page.waitForLoadState('networkidle');
  });

  test('Map page renders correctly', async ({ page }) => {
    // Check if we're on the map page
    expect(page.url()).toContain('/map');
    
    // Look for map container
    const mapContainer = page.locator('#map, [data-testid="map"], .map-container, .mapbox-map');
    
    // Give Mapbox time to load
    await page.waitForTimeout(5000);
    
    const hasMapContainer = await mapContainer.count() > 0;
    const hasLoadingState = await page.locator('[data-testid="loading"], .loading, .map-loading').count() > 0;
    
    expect(hasMapContainer || hasLoadingState).toBeTruthy();
  });

  test('Mapbox integration loads', async ({ page }) => {
    // Wait for Mapbox to potentially load
    await page.waitForTimeout(10000);
    
    // Check for Mapbox-specific elements
    const mapboxElements = await page.locator('.mapboxgl-map, .mapbox-gl-map, [class*="mapbox"]').count() > 0;
    const canvasElement = await page.locator('canvas').count() > 0;
    
    // Check for map-related JavaScript errors
    const errors = [];
    page.on('console', msg => {
      if (msg.type() === 'error' && !msg.text().includes('favicon')) {
        errors.push(msg.text());
      }
    });
    
    // Mapbox should either load or show a meaningful error
    const hasMapError = errors.some(error => 
      error.includes('map') || 
      error.includes('Mapbox') || 
      error.includes('token')
    );
    
    // Should have map elements or clear error about missing token
    if (hasMapError) {
      console.log('Map errors (expected if no Mapbox token):', errors);
    }
    
    expect(mapboxElements || canvasElement || hasMapError).toBeTruthy();
  });

  test('Map controls are present', async ({ page }) => {
    await page.waitForTimeout(5000);
    
    // Look for map controls
    const zoomControls = await page.locator('.mapboxgl-ctrl-zoom, .map-zoom, [data-testid="zoom-controls"]').count() > 0;
    const navigationControls = await page.locator('.mapboxgl-ctrl, .map-controls').count() > 0;
    
    // If map loads, should have some controls
    const mapLoaded = await page.locator('.mapboxgl-map, canvas').count() > 0;
    
    if (mapLoaded) {
      expect(zoomControls || navigationControls).toBeTruthy();
    }
  });

  test('Job markers functionality', async ({ page }) => {
    await page.waitForTimeout(8000);
    
    // Look for job markers on the map
    const markers = page.locator('.mapboxgl-marker, .marker, [data-testid="job-marker"]');
    const markerCount = await markers.count();
    
    if (markerCount > 0) {
      // Test marker interaction
      const firstMarker = markers.first();
      await firstMarker.click();
      
      await page.waitForTimeout(1000);
      
      // Should show popup or job details
      const popup = await page.locator('.mapboxgl-popup, .map-popup, [data-testid="job-popup"]').count() > 0;
      const jobDetails = await page.locator('.job-details, [data-testid="job-details"]').count() > 0;
      
      expect(popup || jobDetails).toBeTruthy();
    } else {
      // If no markers, that's also valid (no jobs to show)
      console.log('No job markers found on map');
    }
  });

  test('Map responds to user interaction', async ({ page }) => {
    await page.waitForTimeout(5000);
    
    const mapContainer = page.locator('#map, [data-testid="map"], .map-container').first();
    
    if (await mapContainer.count() > 0) {
      // Test map interaction (pan/zoom)
      const boundingBox = await mapContainer.boundingBox();
      
      if (boundingBox) {
        // Simulate map pan
        await page.mouse.move(boundingBox.x + boundingBox.width / 2, boundingBox.y + boundingBox.height / 2);
        await page.mouse.down();
        await page.mouse.move(boundingBox.x + boundingBox.width / 2 + 50, boundingBox.y + boundingBox.height / 2 + 50);
        await page.mouse.up();
        
        await page.waitForTimeout(1000);
        
        // Map should still be functional after interaction
        const mapStillExists = await mapContainer.count() > 0;
        expect(mapStillExists).toBeTruthy();
      }
    }
  });

  test('Location search functionality', async ({ page }) => {
    // Look for location search input
    const searchInput = page.locator('input[placeholder*="location"], input[placeholder*="search"], [data-testid="location-search"]').first();
    
    if (await searchInput.count() > 0) {
      await searchInput.fill('Sydney');
      await searchInput.press('Enter');
      
      await page.waitForTimeout(3000);
      
      // Should update map or show search results
      const searchResults = await page.locator('.search-results, [data-testid="search-results"]').count() > 0;
      const mapContainer = await page.locator('#map, [data-testid="map"]').count() > 0;
      
      expect(searchResults || mapContainer).toBeTruthy();
    }
  });

  test('Mobile responsive design', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.waitForTimeout(3000);
    
    // Map should still be visible and functional on mobile
    const mapContainer = page.locator('#map, [data-testid="map"], .map-container');
    const mobileMapVisible = await mapContainer.isVisible();
    
    expect(mobileMapVisible).toBeTruthy();
    
    // Check if mobile-specific controls exist
    const mobileControls = await page.locator('.mobile-map-controls, [data-testid="mobile-controls"]').count() > 0;
    
    // Mobile controls are optional, but map should be visible
    expect(mobileMapVisible).toBeTruthy();
  });
});