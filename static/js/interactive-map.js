/**
 * Interactive Job Map with Advanced Smart Clustering & 3D Visualization
 * Phase 3: Advanced Map Features Implementation
 * Features: Smart clustering, 3D visualization, ML-powered insights
 */

class InteractiveJobMap {
    constructor(containerId, options = {}) {
        this.containerId = containerId;
        this.map = null;
        this.userLocation = null;
        this.jobs = [];
        this.clusters = [];
        this.smartClusters = new Map();
        this.markers = new Map();
        this.selectedJobId = null;
        this.isLoading = false;
        this.visualization3D = false;
        this.clusteringStrategy = 'smart'; // 'smart', 'density', 'salary', 'distance'
        
        // Configuration with enhanced smart clustering options
        this.config = {
            accessToken: options.accessToken || 'pk.eyJ1IjoibG9jb3BsYXRmb3JtIiwiYSI6ImNscTFtb3AwNDAwMDAybHBzNWd1NjdvejEifQ.demo-token', // Demo token
            style: options.style || 'mapbox://styles/mapbox/light-v11',
            center: options.center || [133.7751, -25.2744], // Australia center
            zoom: options.zoom || 5,
            clusterRadius: options.clusterRadius || 50,
            clusterMaxZoom: options.clusterMaxZoom || 14,
            // Smart clustering options
            smartClusterRadius: options.smartClusterRadius || 80,
            densityThreshold: options.densityThreshold || 5,
            salaryClusteringThreshold: options.salaryClusteringThreshold || 20000,
            // 3D visualization options
            enable3D: options.enable3D || false,
            buildingExtrusion: options.buildingExtrusion || true,
            buildingHeight: options.buildingHeight || 50,
            ...options
        };
        
        // Map state
        this.mapState = {
            bounds: null,
            zoom: this.config.zoom,
            center: this.config.center,
            filters: {
                jobTypes: [],
                salaryRange: [0, 200000],
                isUrgent: false,
                radius: 50
            }
        };
        
        this.init();
    }

    /**
     * Initialize the map and all features
     */
    async init() {
        try {
            await this.initializeMap();
            this.setupEventListeners();
            this.setupControls();
            this.setupClustering();
            await this.loadJobData();
            this.setupUserLocation();
            
            console.log('Interactive job map initialized successfully');
        } catch (error) {
            console.error('Failed to initialize map:', error);
            this.showMapError('Failed to load map. Please refresh the page.');
        }
    }

    /**
     * Initialize Mapbox map instance
     */
    async initializeMap() {
        // Check if Mapbox is available
        if (typeof mapboxgl === 'undefined') {
            throw new Error('Mapbox GL JS not loaded');
        }

        // Set access token (use demo mode if no token)
        if (this.config.accessToken.includes('demo')) {
            console.warn('Using demo mode - map functionality limited');
            this.setupDemoMap();
            return;
        }

        mapboxgl.accessToken = this.config.accessToken;

        // Create map instance
        this.map = new mapboxgl.Map({
            container: this.containerId,
            style: this.config.style,
            center: this.config.center,
            zoom: this.config.zoom,
            attributionControl: false
        });

        // Wait for map to load
        await new Promise((resolve, reject) => {
            this.map.on('load', resolve);
            this.map.on('error', reject);
        });

        // Add custom controls
        this.map.addControl(new mapboxgl.NavigationControl(), 'top-right');
        this.map.addControl(new mapboxgl.FullscreenControl(), 'top-right');
        this.map.addControl(new mapboxgl.GeolocateControl({
            positionOptions: {
                enableHighAccuracy: true
            },
            trackUserLocation: true,
            showUserHeading: true
        }), 'top-right');
    }

    /**
     * Setup demo map when Mapbox token is not available
     */
    setupDemoMap() {
        const container = document.getElementById(this.containerId);
        container.innerHTML = `
            <div class="demo-map">
                <div class="demo-map-content">
                    <div class="australia-outline">
                        <svg viewBox="0 0 800 600" class="australia-svg">
                            <!-- Simplified Australia outline -->
                            <path d="M100,200 L200,180 L300,170 L400,160 L500,170 L600,180 L650,200 L700,250 L720,300 L700,350 L650,400 L600,420 L500,430 L400,440 L300,430 L200,420 L150,400 L120,350 L100,300 Z" 
                                  fill="var(--color-primary-100)" 
                                  stroke="var(--color-primary-300)" 
                                  stroke-width="2"/>
                        </svg>
                        
                        <!-- Demo job markers -->
                        <div class="demo-marker" style="top: 25%; left: 75%" data-city="Sydney">
                            <div class="marker-dot urgent"></div>
                            <div class="marker-label">Sydney (12 jobs)</div>
                        </div>
                        
                        <div class="demo-marker" style="top: 35%; left: 65%" data-city="Melbourne">
                            <div class="marker-dot"></div>
                            <div class="marker-label">Melbourne (8 jobs)</div>
                        </div>
                        
                        <div class="demo-marker" style="top: 20%; left: 45%" data-city="Brisbane">
                            <div class="marker-dot"></div>
                            <div class="marker-label">Brisbane (6 jobs)</div>
                        </div>
                        
                        <div class="demo-marker" style="top: 40%; left: 25%" data-city="Perth">
                            <div class="marker-dot"></div>
                            <div class="marker-label">Perth (4 jobs)</div>
                        </div>
                        
                        <div class="demo-marker" style="top: 45%; left: 55%" data-city="Adelaide">
                            <div class="marker-dot"></div>
                            <div class="marker-label">Adelaide (3 jobs)</div>
                        </div>
                    </div>
                </div>
                
                <div class="demo-controls">
                    <button class="map-btn" onclick="jobMap.showCityJobs('Sydney')">
                        <i class="fas fa-eye"></i> View Sydney Jobs
                    </button>
                    <button class="map-btn" onclick="jobMap.showCityJobs('Melbourne')">
                        <i class="fas fa-eye"></i> View Melbourne Jobs
                    </button>
                    <button class="map-btn" onclick="jobMap.enableFullMap()">
                        <i class="fas fa-map"></i> Enable Full Map
                    </button>
                </div>
                
                <div class="map-legend">
                    <div class="legend-item">
                        <div class="legend-dot"></div>
                        <span>Available Jobs</span>
                    </div>
                    <div class="legend-item">
                        <div class="legend-dot urgent"></div>
                        <span>Urgent Hiring</span>
                    </div>
                </div>
            </div>
        `;
        
        this.setupDemoInteractions();
    }

    /**
     * Setup demo map interactions
     */
    setupDemoInteractions() {
        const demoMarkers = document.querySelectorAll('.demo-marker');
        demoMarkers.forEach(marker => {
            marker.addEventListener('click', (e) => {
                const city = e.currentTarget.dataset.city;
                this.showCityJobs(city);
            });
        });
    }

    /**
     * Setup advanced smart clustering for job markers
     */
    setupClustering() {
        if (!this.map) return;

        // Add job data source with smart clustering
        this.map.addSource('jobs', {
            type: 'geojson',
            data: {
                type: 'FeatureCollection',
                features: []
            },
            cluster: true,
            clusterMaxZoom: this.config.clusterMaxZoom,
            clusterRadius: this.config.smartClusterRadius,
            // Enhanced clustering properties
            clusterProperties: {
                'total_jobs': ['+', 1],
                'avg_salary': ['/', ['+', ['get', 'salary']], ['get', 'point_count']],
                'urgent_count': ['+', ['case', ['get', 'isUrgent'], 1, 0]],
                'max_salary': ['max', ['get', 'salary']],
                'min_salary': ['min', ['get', 'salary']]
            }
        });

        // Add smart cluster circles with salary-based coloring
        this.map.addLayer({
            id: 'clusters',
            type: 'circle',
            source: 'jobs',
            filter: ['has', 'point_count'],
            paint: {
                'circle-color': [
                    'case',
                    // High salary cluster (>100k avg)
                    ['>', ['get', 'avg_salary'], 100000], '#2ecc71',
                    // Medium salary cluster (60k-100k avg) 
                    ['>', ['get', 'avg_salary'], 60000], '#f39c12',
                    // Has urgent jobs
                    ['>', ['get', 'urgent_count'], 0], '#e74c3c',
                    // Default
                    '#3498db'
                ],
                'circle-radius': [
                    'interpolate',
                    ['linear'],
                    ['get', 'point_count'],
                    1, 15,   // Single job: 15px
                    5, 25,   // 5 jobs: 25px
                    10, 35,  // 10 jobs: 35px
                    20, 45,  // 20 jobs: 45px
                    50, 60   // 50+ jobs: 60px
                ],
                'circle-opacity': 0.7,
                'circle-stroke-width': 3,
                'circle-stroke-color': '#ffffff',
                // Add pulsing effect for urgent clusters
                'circle-stroke-opacity': [
                    'case',
                    ['>', ['get', 'urgent_count'], 0], 1,
                    0.5
                ]
            }
        });

        // Add enhanced cluster labels with salary info
        this.map.addLayer({
            id: 'cluster-count',
            type: 'symbol',
            source: 'jobs',
            filter: ['has', 'point_count'],
            layout: {
                'text-field': [
                    'format',
                    ['get', 'point_count_abbreviated'], { 'font-scale': 1.2 },
                    '\n',
                    '$', ['round', ['/', ['get', 'avg_salary'], 1000]], 'k avg', { 'font-scale': 0.8 }
                ],
                'text-font': ['DIN Offc Pro Medium', 'Arial Unicode MS Bold'],
                'text-size': 12,
                'text-line-height': 1.2,
                'text-justify': 'center'
            },
            paint: {
                'text-color': '#ffffff',
                'text-halo-color': 'rgba(0,0,0,0.5)',
                'text-halo-width': 1
            }
        });

        // Add individual job markers
        this.map.addLayer({
            id: 'unclustered-point',
            type: 'circle',
            source: 'jobs',
            filter: ['!', ['has', 'point_count']],
            paint: {
                'circle-color': [
                    'case',
                    ['get', 'isUrgent'], '#e74c3c',  // Urgent jobs in red
                    '#3498db'  // Regular jobs in blue
                ],
                'circle-radius': [
                    'case',
                    ['get', 'isUrgent'], 8,  // Urgent jobs larger
                    6  // Regular jobs
                ],
                'circle-stroke-width': 2,
                'circle-stroke-color': '#ffffff',
                'circle-opacity': 0.8
            }
        });

        // Setup cluster interactions
        this.setupClusterInteractions();
        
        // Setup 3D visualization layers if enabled
        if (this.config.enable3D) {
            this.setup3DVisualization();
        }
    }

    /**
     * Setup cluster interaction events
     */
    setupClusterInteractions() {
        if (!this.map) return;

        // Cluster click - zoom in
        this.map.on('click', 'clusters', (e) => {
            const features = this.map.queryRenderedFeatures(e.point, {
                layers: ['clusters']
            });
            
            const clusterId = features[0].properties.cluster_id;
            this.map.getSource('jobs').getClusterExpansionZoom(
                clusterId,
                (err, zoom) => {
                    if (err) return;
                    
                    this.map.easeTo({
                        center: features[0].geometry.coordinates,
                        zoom: zoom
                    });
                }
            );
        });

        // Individual job click - show popup
        this.map.on('click', 'unclustered-point', (e) => {
            const job = e.features[0].properties;
            this.showJobPopup(job, e.lngLat);
        });

        // Hover effects
        this.map.on('mouseenter', 'clusters', () => {
            this.map.getCanvas().style.cursor = 'pointer';
        });

        this.map.on('mouseleave', 'clusters', () => {
            this.map.getCanvas().style.cursor = '';
        });

        this.map.on('mouseenter', 'unclustered-point', () => {
            this.map.getCanvas().style.cursor = 'pointer';
        });

        this.map.on('mouseleave', 'unclustered-point', () => {
            this.map.getCanvas().style.cursor = '';
        });
    }

    /**
     * Setup 3D building visualization
     */
    setup3DVisualization() {
        if (!this.map) return;

        // Enable 3D terrain and building extrusion
        this.map.on('style.load', () => {
            // Add 3D building layer
            this.map.addSource('composite', {
                'url': 'mapbox://mapbox.mapbox-streets-v8',
                'type': 'vector'
            });

            this.map.addLayer({
                'id': '3d-buildings',
                'source': 'composite',
                'source-layer': 'building',
                'filter': ['==', 'extrude', 'true'],
                'type': 'fill-extrusion',
                'minzoom': 12,
                'paint': {
                    'fill-extrusion-color': [
                        'case',
                        ['boolean', ['feature-state', 'hasJobs'], false],
                        '#fbbf24', // Gold for buildings with jobs
                        '#94a3b8'  // Gray for regular buildings
                    ],
                    'fill-extrusion-height': [
                        'interpolate',
                        ['linear'],
                        ['zoom'],
                        12, 0,
                        14, ['*', ['get', 'height'], 1.2],
                        16, ['*', ['get', 'height'], 1.5]
                    ],
                    'fill-extrusion-base': [
                        'interpolate',
                        ['linear'],
                        ['zoom'],
                        12, 0,
                        14, ['get', 'min_height']
                    ],
                    'fill-extrusion-opacity': 0.7
                }
            });

            // Add job density heatmap in 3D
            this.map.addLayer({
                'id': 'job-heatmap-3d',
                'type': 'heatmap',
                'source': 'jobs',
                'maxzoom': 15,
                'paint': {
                    'heatmap-weight': [
                        'interpolate',
                        ['linear'],
                        ['get', 'salary'],
                        30000, 0.1,
                        150000, 1
                    ],
                    'heatmap-intensity': [
                        'interpolate',
                        ['linear'],
                        ['zoom'],
                        0, 1,
                        15, 3
                    ],
                    'heatmap-color': [
                        'interpolate',
                        ['linear'],
                        ['heatmap-density'],
                        0, 'rgba(33,102,172,0)',
                        0.2, 'rgb(103,169,207)',
                        0.4, 'rgb(209,229,240)',
                        0.6, 'rgb(253,219,199)',
                        0.8, 'rgb(239,138,98)',
                        1, 'rgb(178,24,43)'
                    ],
                    'heatmap-radius': [
                        'interpolate',
                        ['linear'],
                        ['zoom'],
                        0, 2,
                        15, 60
                    ],
                    'heatmap-opacity': [
                        'interpolate',
                        ['linear'],
                        ['zoom'],
                        7, 1,
                        15, 0.3
                    ]
                }
            });
        });
    }

    /**
     * Smart clustering algorithm implementation
     */
    generateSmartClusters(jobs, zoomLevel) {
        const clusters = [];
        const processed = new Set();
        
        for (let i = 0; i < jobs.length; i++) {
            if (processed.has(i)) continue;
            
            const currentJob = jobs[i];
            const cluster = {
                id: `cluster-${i}`,
                center: [currentJob.longitude, currentJob.latitude],
                jobs: [currentJob],
                totalJobs: 1,
                avgSalary: currentJob.salary_range_end || 0,
                urgentCount: currentJob.is_urgent ? 1 : 0,
                maxSalary: currentJob.salary_range_end || 0,
                minSalary: currentJob.salary_range_start || 0,
                dominantJobType: currentJob.job_type
            };
            
            processed.add(i);
            
            // Find nearby jobs based on current clustering strategy
            for (let j = i + 1; j < jobs.length; j++) {
                if (processed.has(j)) continue;
                
                const otherJob = jobs[j];
                const distance = this.calculateDistance(
                    currentJob.latitude, currentJob.longitude,
                    otherJob.latitude, otherJob.longitude
                );
                
                const shouldCluster = this.shouldClusterJobs(
                    currentJob, otherJob, distance, zoomLevel
                );
                
                if (shouldCluster) {
                    cluster.jobs.push(otherJob);
                    cluster.totalJobs++;
                    cluster.urgentCount += otherJob.is_urgent ? 1 : 0;
                    
                    // Update center (weighted average)
                    const weight = cluster.jobs.length;
                    cluster.center[0] = (cluster.center[0] * (weight - 1) + otherJob.longitude) / weight;
                    cluster.center[1] = (cluster.center[1] * (weight - 1) + otherJob.latitude) / weight;
                    
                    // Update salary metrics
                    const salaries = cluster.jobs
                        .map(job => job.salary_range_end || 0)
                        .filter(s => s > 0);
                    cluster.avgSalary = salaries.reduce((a, b) => a + b, 0) / salaries.length;
                    cluster.maxSalary = Math.max(...salaries);
                    cluster.minSalary = Math.min(...salaries.filter(s => s > 0));
                    
                    processed.add(j);
                }
            }
            
            clusters.push(cluster);
        }
        
        return clusters;
    }

    /**
     * Determine if two jobs should be clustered together
     */
    shouldClusterJobs(job1, job2, distance, zoomLevel) {
        const baseRadius = this.config.smartClusterRadius * (15 - zoomLevel) / 15;
        
        switch (this.clusteringStrategy) {
            case 'smart':
                // Smart clustering considers distance, salary similarity, and job type
                const salaryDiff = Math.abs(
                    (job1.salary_range_end || 0) - (job2.salary_range_end || 0)
                );
                const salarySimilar = salaryDiff < this.config.salaryClusteringThreshold;
                const typeMatch = job1.job_type === job2.job_type;
                const proximityScore = 1 - (distance / baseRadius);
                
                return distance < baseRadius && 
                       (salarySimilar || typeMatch || proximityScore > 0.8);
                       
            case 'density':
                // Density-based clustering with variable radius
                return distance < baseRadius * (job1.is_urgent || job2.is_urgent ? 1.5 : 1);
                
            case 'salary':
                // Salary-based clustering groups similar salary ranges
                const salaryRange1 = (job1.salary_range_end || 0) - (job1.salary_range_start || 0);
                const salaryRange2 = (job2.salary_range_end || 0) - (job2.salary_range_start || 0);
                const rangeSimilarity = Math.abs(salaryRange1 - salaryRange2) < 20000;
                
                return distance < baseRadius * 1.2 && rangeSimilarity;
                
            case 'distance':
            default:
                return distance < baseRadius;
        }
    }

    /**
     * Calculate distance between two points in kilometers
     */
    calculateDistance(lat1, lng1, lat2, lng2) {
        const R = 6371; // Earth's radius in km
        const dLat = this.toRadians(lat2 - lat1);
        const dLng = this.toRadians(lng2 - lng1);
        const a = Math.sin(dLat / 2) * Math.sin(dLat / 2) +
                  Math.cos(this.toRadians(lat1)) * Math.cos(this.toRadians(lat2)) *
                  Math.sin(dLng / 2) * Math.sin(dLng / 2);
        const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
        return R * c;
    }

    toRadians(degrees) {
        return degrees * (Math.PI / 180);
    }

    /**
     * Toggle between different clustering strategies
     */
    setClusteringStrategy(strategy) {
        this.clusteringStrategy = strategy;
        this.updateMapData();
        this.showNotification(`Clustering strategy: ${strategy}`, 'info');
    }

    /**
     * Toggle 3D visualization
     */
    toggle3DVisualization() {
        this.visualization3D = !this.visualization3D;
        
        if (this.visualization3D) {
            this.setup3DVisualization();
            // Set map pitch for 3D effect
            this.map.easeTo({
                pitch: 45,
                bearing: -17.6,
                duration: 1000
            });
            this.showNotification('3D visualization enabled', 'success');
        } else {
            // Hide 3D layers
            if (this.map.getLayer('3d-buildings')) {
                this.map.setLayoutProperty('3d-buildings', 'visibility', 'none');
            }
            if (this.map.getLayer('job-heatmap-3d')) {
                this.map.setLayoutProperty('job-heatmap-3d', 'visibility', 'none');
            }
            
            // Reset map pitch
            this.map.easeTo({
                pitch: 0,
                bearing: 0,
                duration: 1000
            });
            this.showNotification('3D visualization disabled', 'info');
        }
    }

    /**
     * Load job data from API
     */
    async loadJobData() {
        this.isLoading = true;
        this.showLoadingState();
        
        try {
            const response = await fetch('http://localhost:3070/api/jobs');
            
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            
            const data = await response.json();
            this.jobs = data.jobs || [];
            
            if (this.jobs.length === 0) {
                this.loadDemoJobs();
            }
            
            this.updateMapData();
            this.updateJobStats();
            
        } catch (error) {
            console.warn('Failed to load jobs from API, using demo data:', error);
            this.loadDemoJobs();
        } finally {
            this.isLoading = false;
            this.hideLoadingState();
        }
    }

    /**
     * Load enhanced demo job data with 3D building context
     */
    loadDemoJobs() {
        this.jobs = [
            // Sydney CBD cluster - High salary zone
            {
                id: 'demo-1',
                title: 'Senior Pharmacist',
                company: 'Sydney Pharmacy Group',
                location: 'Sydney CBD, NSW',
                latitude: -33.8688,
                longitude: 151.2093,
                salary_range_start: 120000,
                salary_range_end: 140000,
                is_urgent: true,
                job_type: 'FullTime',
                description: 'Leading role in busy city pharmacy.',
                building_height: 150,
                floor_level: 12
            },
            {
                id: 'demo-1b',
                title: 'Clinical Pharmacist',
                company: 'Harbour Medical Centre',
                location: 'Sydney CBD, NSW',
                latitude: -33.8698,
                longitude: 151.2083,
                salary_range_start: 110000,
                salary_range_end: 130000,
                is_urgent: false,
                job_type: 'FullTime',
                description: 'Clinical role in prestigious medical centre.',
                building_height: 200,
                floor_level: 8
            },
            {
                id: 'demo-1c',
                title: 'Pharmacy Director',
                company: 'CBD Health Group',
                location: 'Sydney CBD, NSW',
                latitude: -33.8678,
                longitude: 151.2103,
                salary_range_start: 150000,
                salary_range_end: 180000,
                is_urgent: true,
                job_type: 'FullTime',
                description: 'Executive leadership role.',
                building_height: 300,
                floor_level: 25
            },
            
            // Melbourne cluster - Medium salary zone
            {
                id: 'demo-2',
                title: 'Locum Pharmacist',
                company: 'Melbourne Community Pharmacy',
                location: 'Melbourne, VIC',
                latitude: -37.8136,
                longitude: 144.9631,
                salary_range_start: 55000,
                salary_range_end: 75000,
                is_urgent: false,
                job_type: 'Contract',
                description: 'Flexible locum position available.',
                building_height: 80,
                floor_level: 2
            },
            {
                id: 'demo-2b',
                title: 'Retail Pharmacist',
                company: 'Collins Street Pharmacy',
                location: 'Melbourne, VIC',
                latitude: -37.8146,
                longitude: 144.9641,
                salary_range_start: 65000,
                salary_range_end: 85000,
                is_urgent: false,
                job_type: 'FullTime',
                description: 'Busy retail pharmacy role.',
                building_height: 120,
                floor_level: 1
            },
            
            // Brisbane cluster
            {
                id: 'demo-3',
                title: 'Hospital Pharmacist',
                company: 'Brisbane General Hospital',
                location: 'Brisbane, QLD',
                latitude: -27.4698,
                longitude: 153.0251,
                salary_range_start: 90000,
                salary_range_end: 110000,
                is_urgent: false,
                job_type: 'FullTime',
                description: 'Hospital pharmacy role focusing on clinical services.',
                building_height: 180,
                floor_level: 6
            },
            {
                id: 'demo-3b',
                title: 'Emergency Pharmacist',
                company: 'Brisbane Emergency Medical',
                location: 'Brisbane, QLD',
                latitude: -27.4708,
                longitude: 153.0261,
                salary_range_start: 95000,
                salary_range_end: 115000,
                is_urgent: true,
                job_type: 'FullTime',
                description: 'Critical care pharmacy position.',
                building_height: 220,
                floor_level: 3
            },
            
            // Perth cluster
            {
                id: 'demo-4',
                title: 'Pharmacy Manager',
                company: 'Perth Pharmacy Chain',
                location: 'Perth, WA',
                latitude: -31.9505,
                longitude: 115.8605,
                salary_range_start: 130000,
                salary_range_end: 150000,
                is_urgent: false,
                job_type: 'FullTime',
                description: 'Management opportunity for experienced pharmacist.',
                building_height: 100,
                floor_level: 4
            },
            
            // Adelaide cluster
            {
                id: 'demo-5',
                title: 'Graduate Pharmacist',
                company: 'Adelaide Family Pharmacy',
                location: 'Adelaide, SA',
                latitude: -34.9285,
                longitude: 138.6007,
                salary_range_start: 65000,
                salary_range_end: 80000,
                is_urgent: false,
                job_type: 'PartTime',
                description: 'Excellent opportunity for new graduate.',
                building_height: 60,
                floor_level: 1
            },
            
            // Gold Coast cluster - New high-value area
            {
                id: 'demo-6',
                title: 'Resort Pharmacist',
                company: 'Gold Coast Resort Health',
                location: 'Gold Coast, QLD',
                latitude: -28.0167,
                longitude: 153.4000,
                salary_range_start: 105000,
                salary_range_end: 125000,
                is_urgent: true,
                job_type: 'FullTime',
                description: 'Luxury resort pharmacy position.',
                building_height: 250,
                floor_level: 15
            }
        ];
        
        this.updateMapData();
        this.updateJobStats();
    }

    /**
     * Update map with job data using smart clustering
     */
    updateMapData() {
        const features = this.jobs
            .filter(job => job.latitude && job.longitude)
            .map(job => ({
                type: 'Feature',
                properties: {
                    id: job.id,
                    title: job.title,
                    company: job.company,
                    location: job.location,
                    salaryStart: job.salary_range_start,
                    salaryEnd: job.salary_range_end,
                    salary: job.salary_range_end || job.salary_range_start || 0, // For clustering calculations
                    isUrgent: job.is_urgent,
                    jobType: job.job_type,
                    description: job.description
                },
                geometry: {
                    type: 'Point',
                    coordinates: [job.longitude, job.latitude]
                }
            }));

        if (this.map && this.map.getSource('jobs')) {
            this.map.getSource('jobs').setData({
                type: 'FeatureCollection',
                features: features
            });
        }
        
        // Generate smart clusters for analysis
        if (this.map) {
            const currentZoom = this.map.getZoom();
            this.smartClusters = this.generateSmartClusters(this.jobs, currentZoom);
            this.updateClusterAnalysis();
        }
    }

    /**
     * Update cluster analysis display
     */
    updateClusterAnalysis() {
        const analysisEl = document.getElementById('cluster-analysis');
        if (!analysisEl) return;
        
        const totalClusters = this.smartClusters.length;
        const avgJobsPerCluster = this.smartClusters.reduce((sum, c) => sum + c.totalJobs, 0) / totalClusters;
        const highSalaryClusters = this.smartClusters.filter(c => c.avgSalary > 100000).length;
        const urgentClusters = this.smartClusters.filter(c => c.urgentCount > 0).length;
        
        analysisEl.innerHTML = `
            <div class="cluster-stats">
                <div class="stat-item">
                    <span class="stat-label">Smart Clusters</span>
                    <span class="stat-value">${totalClusters}</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">Avg Jobs/Cluster</span>
                    <span class="stat-value">${avgJobsPerCluster.toFixed(1)}</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">High Salary Zones</span>
                    <span class="stat-value">${highSalaryClusters}</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">Urgent Clusters</span>
                    <span class="stat-value">${urgentClusters}</span>
                </div>
            </div>
        `;
    }

    /**
     * Show job popup
     */
    showJobPopup(job, coordinates) {
        const salaryDisplay = this.formatSalary(job.salaryStart, job.salaryEnd);
        
        const popupContent = `
            <div class="job-popup">
                <div class="job-popup-header">
                    <h3 class="job-title">${job.title}</h3>
                    ${job.isUrgent ? '<span class="urgent-badge">Urgent</span>' : ''}
                </div>
                
                <div class="job-popup-body">
                    <div class="company">${job.company}</div>
                    <div class="location">${job.location}</div>
                    <div class="salary">${salaryDisplay}</div>
                    <div class="job-type">${job.jobType}</div>
                    
                    <p class="job-description">${this.truncateText(job.description, 100)}</p>
                </div>
                
                <div class="job-popup-footer">
                    <button class="btn btn-outline btn-sm" onclick="jobMap.saveJob('${job.id}')">
                        <i class="fas fa-bookmark"></i> Save
                    </button>
                    <button class="btn btn-primary btn-sm" onclick="jobMap.viewJobDetails('${job.id}')">
                        View Details
                    </button>
                </div>
            </div>
        `;

        if (this.map) {
            new mapboxgl.Popup({ offset: 25 })
                .setLngLat(coordinates)
                .setHTML(popupContent)
                .addTo(this.map);
        } else {
            // Demo mode popup
            this.showDemoPopup(job);
        }
    }

    /**
     * Show demo popup for demo mode
     */
    showDemoPopup(job) {
        const existingPopup = document.querySelector('.demo-popup');
        if (existingPopup) {
            existingPopup.remove();
        }

        const salaryDisplay = this.formatSalary(job.salaryStart, job.salaryEnd);
        
        const popup = document.createElement('div');
        popup.className = 'demo-popup glass-card';
        popup.innerHTML = `
            <div class="demo-popup-content">
                <button class="demo-popup-close" onclick="this.parentElement.parentElement.remove()">
                    <i class="fas fa-times"></i>
                </button>
                
                <div class="job-popup">
                    <div class="job-popup-header">
                        <h3 class="job-title">${job.title}</h3>
                        ${job.isUrgent ? '<span class="urgent-badge">Urgent</span>' : ''}
                    </div>
                    
                    <div class="job-popup-body">
                        <div class="company">${job.company}</div>
                        <div class="location">${job.location}</div>
                        <div class="salary">${salaryDisplay}</div>
                        <div class="job-type">${job.jobType}</div>
                        
                        <p class="job-description">${job.description}</p>
                    </div>
                    
                    <div class="job-popup-footer">
                        <button class="btn btn-outline btn-sm" onclick="jobMap.saveJob('${job.id}')">
                            <i class="fas fa-bookmark"></i> Save
                        </button>
                        <button class="btn btn-primary btn-sm" onclick="jobMap.viewJobDetails('${job.id}')">
                            View Details
                        </button>
                    </div>
                </div>
            </div>
        `;
        
        document.body.appendChild(popup);
    }

    /**
     * Setup event listeners
     */
    setupEventListeners() {
        // Window resize
        window.addEventListener('resize', () => {
            if (this.map) {
                this.map.resize();
            }
        });

        // Filter controls
        this.setupFilterControls();
    }

    /**
     * Setup filter controls
     */
    setupFilterControls() {
        // Job type filters
        const jobTypeFilters = document.querySelectorAll('[name="map-job-type"]');
        jobTypeFilters.forEach(filter => {
            filter.addEventListener('change', () => {
                this.updateFilters();
            });
        });

        // Urgent jobs filter
        const urgentFilter = document.getElementById('map-urgent-only');
        if (urgentFilter) {
            urgentFilter.addEventListener('change', () => {
                this.updateFilters();
            });
        }

        // Salary range filter
        const salaryFilter = document.getElementById('map-salary-range');
        if (salaryFilter) {
            salaryFilter.addEventListener('input', () => {
                this.updateFilters();
            });
        }
    }

    /**
     * Update map filters
     */
    updateFilters() {
        const selectedJobTypes = Array.from(document.querySelectorAll('[name="map-job-type"]:checked'))
            .map(cb => cb.value);
        
        const urgentOnly = document.getElementById('map-urgent-only')?.checked || false;
        const salaryMin = parseInt(document.getElementById('map-salary-range')?.value || 0);

        this.mapState.filters = {
            jobTypes: selectedJobTypes,
            isUrgent: urgentOnly,
            salaryMin: salaryMin
        };

        this.applyFilters();
    }

    /**
     * Apply filters to map data
     */
    applyFilters() {
        const filteredJobs = this.jobs.filter(job => {
            // Job type filter
            if (this.mapState.filters.jobTypes.length > 0 && 
                !this.mapState.filters.jobTypes.includes(job.job_type)) {
                return false;
            }

            // Urgent filter
            if (this.mapState.filters.isUrgent && !job.is_urgent) {
                return false;
            }

            // Salary filter
            if (this.mapState.filters.salaryMin > 0 && 
                (!job.salary_range_end || job.salary_range_end < this.mapState.filters.salaryMin)) {
                return false;
            }

            return true;
        });

        // Update map with filtered data
        const features = filteredJobs
            .filter(job => job.latitude && job.longitude)
            .map(job => ({
                type: 'Feature',
                properties: {
                    id: job.id,
                    title: job.title,
                    company: job.company,
                    location: job.location,
                    salaryStart: job.salary_range_start,
                    salaryEnd: job.salary_range_end,
                    isUrgent: job.is_urgent,
                    jobType: job.job_type,
                    description: job.description
                },
                geometry: {
                    type: 'Point',
                    coordinates: [job.longitude, job.latitude]
                }
            }));

        if (this.map && this.map.getSource('jobs')) {
            this.map.getSource('jobs').setData({
                type: 'FeatureCollection',
                features: features
            });
        }

        this.updateJobStats(filteredJobs.length);
    }

    /**
     * Setup user location tracking
     */
    setupUserLocation() {
        if (!navigator.geolocation) return;

        const locationButton = document.getElementById('find-my-location');
        if (locationButton) {
            locationButton.addEventListener('click', () => {
                this.goToUserLocation();
            });
        }
    }

    /**
     * Go to user's current location
     */
    goToUserLocation() {
        if (!navigator.geolocation) {
            this.showNotification('Geolocation not supported', 'error');
            return;
        }

        navigator.geolocation.getCurrentPosition(
            (position) => {
                const coords = [position.coords.longitude, position.coords.latitude];
                this.userLocation = coords;

                if (this.map) {
                    // Add user location marker
                    this.addUserLocationMarker(coords);
                    
                    // Fly to user location
                    this.map.flyTo({
                        center: coords,
                        zoom: 12,
                        duration: 2000
                    });
                }

                this.showNotification('Location found!', 'success');
            },
            (error) => {
                console.error('Geolocation error:', error);
                this.showNotification('Could not get your location', 'error');
            }
        );
    }

    /**
     * Add user location marker
     */
    addUserLocationMarker(coordinates) {
        if (!this.map) return;

        // Remove existing user marker
        if (this.userMarker) {
            this.userMarker.remove();
        }

        // Create user marker element
        const el = document.createElement('div');
        el.className = 'user-marker';
        el.style.width = '20px';
        el.style.height = '20px';
        el.style.borderRadius = '50%';
        el.style.backgroundColor = '#3b82f6';
        el.style.border = '3px solid white';
        el.style.boxShadow = '0 0 10px rgba(0,0,0,0.3)';

        // Add marker to map
        this.userMarker = new mapboxgl.Marker(el)
            .setLngLat(coordinates)
            .addTo(this.map);
    }

    /**
     * Setup map controls
     */
    setupControls() {
        this.setupViewControls();
        this.setupSearchIntegration();
    }

    /**
     * Setup view controls
     */
    setupViewControls() {
        // Zoom to fit all jobs
        const fitJobsButton = document.getElementById('fit-all-jobs');
        if (fitJobsButton) {
            fitJobsButton.addEventListener('click', () => {
                this.fitAllJobs();
            });
        }

        // Toggle clustering
        const clusterToggle = document.getElementById('toggle-clustering');
        if (clusterToggle) {
            clusterToggle.addEventListener('change', (e) => {
                this.toggleClustering(e.target.checked);
            });
        }
    }

    /**
     * Fit map to show all jobs
     */
    fitAllJobs() {
        if (!this.map || this.jobs.length === 0) return;

        const coordinates = this.jobs
            .filter(job => job.latitude && job.longitude)
            .map(job => [job.longitude, job.latitude]);

        if (coordinates.length === 0) return;

        const bounds = coordinates.reduce((bounds, coord) => {
            return bounds.extend(coord);
        }, new mapboxgl.LngLatBounds(coordinates[0], coordinates[0]));

        this.map.fitBounds(bounds, {
            padding: 50,
            duration: 1500
        });
    }

    /**
     * Toggle clustering on/off
     */
    toggleClustering(enabled) {
        if (!this.map) return;

        const source = this.map.getSource('jobs');
        if (source) {
            // Update source clustering property
            source.setData({
                ...source._data,
                cluster: enabled
            });
        }
    }

    /**
     * Setup search integration with job search
     */
    setupSearchIntegration() {
        // Listen for search events from main search
        window.addEventListener('mapSearchUpdate', (event) => {
            const { jobs, filters } = event.detail;
            this.jobs = jobs;
            this.mapState.filters = { ...this.mapState.filters, ...filters };
            this.updateMapData();
        });
    }

    /**
     * Utility functions
     */
    formatSalary(min, max) {
        if (min && max) {
            return `$${min.toLocaleString()} - $${max.toLocaleString()}`;
        } else if (min) {
            return `From $${min.toLocaleString()}`;
        } else if (max) {
            return `Up to $${max.toLocaleString()}`;
        }
        return 'Competitive';
    }

    truncateText(text, maxLength) {
        if (text.length <= maxLength) return text;
        return text.substring(0, maxLength) + '...';
    }

    updateJobStats(count = null) {
        const jobCount = count !== null ? count : this.jobs.length;
        const statsElement = document.getElementById('map-job-count');
        if (statsElement) {
            statsElement.textContent = `${jobCount} jobs shown`;
        }
    }

    showLoadingState() {
        const loader = document.getElementById('map-loader');
        if (loader) {
            loader.style.display = 'block';
        }
    }

    hideLoadingState() {
        const loader = document.getElementById('map-loader');
        if (loader) {
            loader.style.display = 'none';
        }
    }

    showNotification(message, type = 'info') {
        // Create notification element
        const notification = document.createElement('div');
        notification.className = `map-notification notification-${type}`;
        notification.textContent = message;
        
        document.body.appendChild(notification);
        
        setTimeout(() => {
            notification.remove();
        }, 3000);
    }

    showMapError(message) {
        const container = document.getElementById(this.containerId);
        container.innerHTML = `
            <div class="map-error">
                <i class="fas fa-exclamation-triangle text-6xl text-orange-400 mb-4"></i>
                <h3 class="text-xl font-semibold mb-2">Map Error</h3>
                <p class="text-gray-600">${message}</p>
                <button onclick="location.reload()" class="btn btn-primary mt-4">
                    Reload Page
                </button>
            </div>
        `;
    }

    // Demo mode specific methods
    showCityJobs(city) {
        // Filter jobs by city and show them
        const cityJobs = this.jobs.filter(job => 
            job.location.toLowerCase().includes(city.toLowerCase())
        );
        
        if (cityJobs.length > 0) {
            // Show job list for the city
            this.showJobsList(cityJobs, city);
        } else {
            this.showNotification(`No jobs found in ${city}`, 'info');
        }
    }

    showJobsList(jobs, city) {
        const jobsHTML = jobs.map(job => `
            <div class="city-job-item" onclick="jobMap.viewJobDetails('${job.id}')">
                <div class="job-title">${job.title}</div>
                <div class="job-company">${job.company}</div>
                <div class="job-salary">${this.formatSalary(job.salary_range_start, job.salary_range_end)}</div>
                ${job.is_urgent ? '<span class="urgent-badge">Urgent</span>' : ''}
            </div>
        `).join('');

        const popup = document.createElement('div');
        popup.className = 'city-jobs-popup glass-card';
        popup.innerHTML = `
            <div class="city-jobs-content">
                <div class="city-jobs-header">
                    <h3>${city} Jobs (${jobs.length})</h3>
                    <button onclick="this.closest('.city-jobs-popup').remove()">
                        <i class="fas fa-times"></i>
                    </button>
                </div>
                <div class="city-jobs-list">
                    ${jobsHTML}
                </div>
            </div>
        `;
        
        document.body.appendChild(popup);
    }

    enableFullMap() {
        this.showNotification('Full interactive map requires Mapbox access token', 'info');
    }

    // Job actions
    saveJob(jobId) {
        this.showNotification('Job saved!', 'success');
        // TODO: Implement actual save functionality
    }

    viewJobDetails(jobId) {
        window.location.href = `/job-detail.html?id=${jobId}`;
    }
}

// CSS for demo mode and enhanced styling
const mapStyles = `
<style>
.demo-map {
    width: 100%;
    height: 600px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: var(--radius-xl);
    position: relative;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
}

.demo-map-content {
    position: relative;
    width: 80%;
    height: 80%;
}

.australia-outline {
    position: relative;
    width: 100%;
    height: 100%;
}

.australia-svg {
    width: 100%;
    height: 100%;
    filter: drop-shadow(0 4px 20px rgba(0,0,0,0.1));
}

.demo-marker {
    position: absolute;
    transform: translate(-50%, -50%);
    cursor: pointer;
    transition: all 0.3s ease;
}

.demo-marker:hover {
    transform: translate(-50%, -50%) scale(1.1);
}

.marker-dot {
    width: 12px;
    height: 12px;
    background: var(--color-primary-500);
    border: 2px solid white;
    border-radius: 50%;
    box-shadow: 0 2px 10px rgba(0,0,0,0.2);
    margin: 0 auto 4px;
}

.marker-dot.urgent {
    background: var(--color-red-500);
    animation: pulse 2s infinite;
}

.marker-label {
    background: rgba(255,255,255,0.95);
    backdrop-filter: blur(10px);
    padding: 4px 8px;
    border-radius: var(--radius-md);
    font-size: 0.75rem;
    font-weight: 500;
    white-space: nowrap;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

.demo-controls {
    position: absolute;
    bottom: 20px;
    left: 20px;
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
}

.map-btn {
    background: rgba(255,255,255,0.9);
    backdrop-filter: blur(10px);
    border: none;
    padding: 8px 16px;
    border-radius: var(--radius-lg);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

.map-btn:hover {
    background: rgba(255,255,255,1);
    transform: translateY(-2px);
    box-shadow: 0 4px 20px rgba(0,0,0,0.15);
}

.map-legend {
    position: absolute;
    top: 20px;
    right: 20px;
    background: rgba(255,255,255,0.9);
    backdrop-filter: blur(10px);
    padding: 16px;
    border-radius: var(--radius-lg);
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

.legend-item {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
    font-size: 0.875rem;
}

.legend-item:last-child {
    margin-bottom: 0;
}

.legend-dot {
    width: 12px;
    height: 12px;
    background: var(--color-primary-500);
    border: 2px solid white;
    border-radius: 50%;
}

.legend-dot.urgent {
    background: var(--color-red-500);
}

.demo-popup {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 1000;
    max-width: 400px;
    width: 90%;
}

.demo-popup-content {
    position: relative;
}

.demo-popup-close {
    position: absolute;
    top: 16px;
    right: 16px;
    background: none;
    border: none;
    font-size: 1.2rem;
    color: var(--color-gray-500);
    cursor: pointer;
    z-index: 1;
}

.city-jobs-popup {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 1000;
    max-width: 500px;
    width: 90%;
    max-height: 80vh;
    overflow: hidden;
}

.city-jobs-content {
    position: relative;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.city-jobs-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px;
    border-bottom: 1px solid var(--color-gray-200);
}

.city-jobs-header button {
    background: none;
    border: none;
    font-size: 1.2rem;
    color: var(--color-gray-500);
    cursor: pointer;
}

.city-jobs-list {
    flex: 1;
    overflow-y: auto;
    padding: 20px;
}

.city-job-item {
    padding: 16px;
    border: 1px solid var(--color-gray-200);
    border-radius: var(--radius-lg);
    margin-bottom: 12px;
    cursor: pointer;
    transition: all 0.3s ease;
    position: relative;
}

.city-job-item:hover {
    border-color: var(--color-primary-300);
    box-shadow: 0 4px 20px rgba(0,0,0,0.1);
    transform: translateY(-2px);
}

.job-popup {
    padding: 20px;
}

.job-popup-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 16px;
}

.job-title {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--color-gray-900);
    margin: 0;
}

.urgent-badge {
    background: var(--color-red-500);
    color: white;
    padding: 4px 8px;
    border-radius: var(--radius-md);
    font-size: 0.75rem;
    font-weight: 500;
}

.job-popup-body {
    margin-bottom: 16px;
}

.job-popup-body > div {
    margin-bottom: 8px;
    color: var(--color-gray-600);
    font-size: 0.875rem;
}

.company {
    font-weight: 500;
    color: var(--color-gray-700);
}

.salary {
    font-weight: 600;
    color: var(--color-green-600);
}

.job-description {
    color: var(--color-gray-600);
    line-height: 1.5;
    margin: 12px 0;
}

.job-popup-footer {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
}

.map-error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 600px;
    text-align: center;
    color: var(--color-gray-600);
}

.map-notification {
    position: fixed;
    top: 20px;
    right: 20px;
    padding: 12px 16px;
    border-radius: var(--radius-lg);
    color: white;
    font-weight: 500;
    z-index: 1000;
    animation: slideIn 0.3s ease;
}

.notification-success {
    background: var(--color-green-500);
}

.notification-error {
    background: var(--color-red-500);
}

.notification-info {
    background: var(--color-blue-500);
}

@keyframes slideIn {
    from {
        transform: translateX(100%);
        opacity: 0;
    }
    to {
        transform: translateX(0);
        opacity: 1;
    }
}

@media (max-width: 768px) {
    .demo-controls {
        position: static;
        margin-top: 20px;
        justify-content: center;
    }
    
    .map-legend {
        position: static;
        margin-top: 20px;
        display: inline-block;
    }
    
    .demo-map {
        flex-direction: column;
        padding: 20px;
    }
}
</style>
`;

// Inject styles
document.head.insertAdjacentHTML('beforeend', mapStyles);

// Initialize map when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.jobMap = new InteractiveJobMap('map', {
        // Configuration options
        clusterRadius: 50,
        clusterMaxZoom: 14
    });
});