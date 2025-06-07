/**
 * Advanced Search Module for Loco Platform
 * Handles sophisticated job search and filtering with real-time updates
 */

class AdvancedSearchManager {
    constructor() {
        this.searchState = {
            query: '',
            filters: {
                jobTypes: [],
                locations: [],
                salaryMin: null,
                salaryMax: null,
                isUrgent: false,
                remotePossible: false,
                radiusKm: 50,
                latitude: null,
                longitude: null
            },
            sorting: {
                sortBy: 'relevance',
                sortOrder: 'desc'
            },
            pagination: {
                page: 1,
                limit: 20
            }
        };
        
        this.searchHistory = this.loadSearchHistory();
        this.suggestions = [];
        this.isSearching = false;
        this.searchTimeout = null;
        
        this.init();
    }

    init() {
        this.setupSearchBar();
        this.setupFilters();
        this.setupSorting();
        this.setupPagination();
        this.setupGeolocation();
        this.loadTrendingSearches();
        
        // Load initial results
        this.performSearch();
    }

    /**
     * Setup search bar with autocomplete and suggestions
     */
    setupSearchBar() {
        const searchInput = document.getElementById('search-input');
        const searchButton = document.getElementById('search-button');
        const clearButton = document.getElementById('clear-search');
        const suggestionsContainer = document.getElementById('search-suggestions');

        if (!searchInput) return;

        // Real-time search suggestions
        searchInput.addEventListener('input', (e) => {
            const query = e.target.value.trim();
            this.searchState.query = query;
            
            // Clear previous timeout
            if (this.searchTimeout) {
                clearTimeout(this.searchTimeout);
            }
            
            // Debounced search
            this.searchTimeout = setTimeout(() => {
                if (query.length >= 2) {
                    this.fetchSuggestions(query);
                } else {
                    this.hideSuggestions();
                }
            }, 300);
        });

        // Handle suggestion clicks
        if (suggestionsContainer) {
            suggestionsContainer.addEventListener('click', (e) => {
                if (e.target.classList.contains('suggestion-item')) {
                    const suggestion = e.target.textContent;
                    searchInput.value = suggestion;
                    this.searchState.query = suggestion;
                    this.hideSuggestions();
                    this.performSearch();
                }
            });
        }

        // Search button
        if (searchButton) {
            searchButton.addEventListener('click', () => {
                this.performSearch();
            });
        }

        // Clear search
        if (clearButton) {
            clearButton.addEventListener('click', () => {
                this.clearSearch();
            });
        }

        // Enter key search
        searchInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                this.hideSuggestions();
                this.performSearch();
            }
        });

        // Hide suggestions when clicking outside
        document.addEventListener('click', (e) => {
            if (!searchInput.contains(e.target) && !suggestionsContainer?.contains(e.target)) {
                this.hideSuggestions();
            }
        });
    }

    /**
     * Setup advanced filters
     */
    setupFilters() {
        // Job type filters
        const jobTypeCheckboxes = document.querySelectorAll('[name="job-type"]');
        jobTypeCheckboxes.forEach(checkbox => {
            checkbox.addEventListener('change', (e) => {
                if (e.target.checked) {
                    this.searchState.filters.jobTypes.push(e.target.value);
                } else {
                    this.searchState.filters.jobTypes = this.searchState.filters.jobTypes
                        .filter(type => type !== e.target.value);
                }
                this.performSearch();
            });
        });

        // Salary range sliders
        const salaryMinSlider = document.getElementById('salary-min');
        const salaryMaxSlider = document.getElementById('salary-max');
        const salaryMinDisplay = document.getElementById('salary-min-display');
        const salaryMaxDisplay = document.getElementById('salary-max-display');

        if (salaryMinSlider) {
            salaryMinSlider.addEventListener('input', (e) => {
                const value = parseInt(e.target.value);
                this.searchState.filters.salaryMin = value;
                if (salaryMinDisplay) {
                    salaryMinDisplay.textContent = `$${value.toLocaleString()}`;
                }
                this.debounceSearch();
            });
        }

        if (salaryMaxSlider) {
            salaryMaxSlider.addEventListener('input', (e) => {
                const value = parseInt(e.target.value);
                this.searchState.filters.salaryMax = value;
                if (salaryMaxDisplay) {
                    salaryMaxDisplay.textContent = `$${value.toLocaleString()}`;
                }
                this.debounceSearch();
            });
        }

        // Location radius
        const radiusSlider = document.getElementById('radius-slider');
        const radiusDisplay = document.getElementById('radius-display');

        if (radiusSlider) {
            radiusSlider.addEventListener('input', (e) => {
                const value = parseInt(e.target.value);
                this.searchState.filters.radiusKm = value;
                if (radiusDisplay) {
                    radiusDisplay.textContent = `${value} km`;
                }
                this.debounceSearch();
            });
        }

        // Boolean filters
        const urgentFilter = document.getElementById('urgent-only');
        const remoteFilter = document.getElementById('remote-possible');

        if (urgentFilter) {
            urgentFilter.addEventListener('change', (e) => {
                this.searchState.filters.isUrgent = e.target.checked;
                this.performSearch();
            });
        }

        if (remoteFilter) {
            remoteFilter.addEventListener('change', (e) => {
                this.searchState.filters.remotePossible = e.target.checked;
                this.performSearch();
            });
        }

        // State/location filters
        const stateCheckboxes = document.querySelectorAll('[name="state"]');
        stateCheckboxes.forEach(checkbox => {
            checkbox.addEventListener('change', (e) => {
                if (e.target.checked) {
                    this.searchState.filters.locations.push(e.target.value);
                } else {
                    this.searchState.filters.locations = this.searchState.filters.locations
                        .filter(loc => loc !== e.target.value);
                }
                this.performSearch();
            });
        });
    }

    /**
     * Setup sorting controls
     */
    setupSorting() {
        const sortSelect = document.getElementById('sort-select');
        
        if (sortSelect) {
            sortSelect.addEventListener('change', (e) => {
                const [sortBy, sortOrder] = e.target.value.split('-');
                this.searchState.sorting.sortBy = sortBy;
                this.searchState.sorting.sortOrder = sortOrder;
                this.performSearch();
            });
        }
    }

    /**
     * Setup pagination
     */
    setupPagination() {
        const paginationContainer = document.getElementById('pagination');
        
        if (paginationContainer) {
            paginationContainer.addEventListener('click', (e) => {
                if (e.target.classList.contains('page-btn')) {
                    const page = parseInt(e.target.dataset.page);
                    this.searchState.pagination.page = page;
                    this.performSearch();
                }
            });
        }
    }

    /**
     * Setup geolocation for location-based search
     */
    setupGeolocation() {
        const useLocationBtn = document.getElementById('use-location');
        
        if (useLocationBtn) {
            useLocationBtn.addEventListener('click', () => {
                this.getCurrentLocation();
            });
        }
    }

    /**
     * Get user's current location
     */
    getCurrentLocation() {
        if (!navigator.geolocation) {
            this.showNotification('Geolocation is not supported by this browser', 'error');
            return;
        }

        const locationBtn = document.getElementById('use-location');
        if (locationBtn) {
            locationBtn.textContent = 'Getting location...';
            locationBtn.disabled = true;
        }

        navigator.geolocation.getCurrentPosition(
            (position) => {
                this.searchState.filters.latitude = position.coords.latitude;
                this.searchState.filters.longitude = position.coords.longitude;
                
                if (locationBtn) {
                    locationBtn.textContent = 'Location enabled ✓';
                    locationBtn.classList.add('location-enabled');
                }
                
                this.showNotification('Location enabled - showing nearby jobs first', 'success');
                this.performSearch();
            },
            (error) => {
                console.error('Geolocation error:', error);
                this.showNotification('Could not get your location', 'error');
                
                if (locationBtn) {
                    locationBtn.textContent = 'Use My Location';
                    locationBtn.disabled = false;
                }
            },
            {
                enableHighAccuracy: true,
                timeout: 10000,
                maximumAge: 300000 // 5 minutes
            }
        );
    }

    /**
     * Perform advanced search with all filters
     */
    async performSearch() {
        if (this.isSearching) return;
        
        this.isSearching = true;
        this.showLoading(true);
        
        try {
            const searchParams = {
                query: this.searchState.query || null,
                job_types: this.searchState.filters.jobTypes.length > 0 
                    ? this.searchState.filters.jobTypes.map(type => ({ [type]: type }))
                    : null,
                locations: this.searchState.filters.locations.length > 0 
                    ? this.searchState.filters.locations 
                    : null,
                min_salary: this.searchState.filters.salaryMin,
                max_salary: this.searchState.filters.salaryMax,
                is_urgent: this.searchState.filters.isUrgent || null,
                remote_possible: this.searchState.filters.remotePossible || null,
                latitude: this.searchState.filters.latitude,
                longitude: this.searchState.filters.longitude,
                radius_km: this.searchState.filters.radiusKm,
                page: this.searchState.pagination.page,
                limit: this.searchState.pagination.limit
            };

            const response = await fetch('http://localhost:3070/api/v1/search/advanced', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(searchParams)
            });

            if (!response.ok) {
                throw new Error(`Search failed: ${response.status}`);
            }

            const data = await response.json();
            
            this.displayResults(data.jobs, data.analytics, data.pagination);
            this.updateSearchAnalytics(data.analytics);
            this.saveSearchToHistory();
            
        } catch (error) {
            console.error('Search error:', error);
            this.showNotification('Search failed. Please try again.', 'error');
            this.displayError('Failed to perform search. Please check your connection and try again.');
        } finally {
            this.isSearching = false;
            this.showLoading(false);
        }
    }

    /**
     * Fetch search suggestions
     */
    async fetchSuggestions(query) {
        try {
            const response = await fetch(`http://localhost:3070/api/v1/search/suggestions?q=${encodeURIComponent(query)}`);
            const data = await response.json();
            
            this.suggestions = data.suggestions || [];
            this.displaySuggestions();
            
        } catch (error) {
            console.error('Failed to fetch suggestions:', error);
        }
    }

    /**
     * Display search suggestions
     */
    displaySuggestions() {
        const container = document.getElementById('search-suggestions');
        if (!container) return;

        if (this.suggestions.length === 0) {
            container.style.display = 'none';
            return;
        }

        const suggestionsHTML = this.suggestions.map(suggestion => 
            `<div class="suggestion-item">${suggestion}</div>`
        ).join('');

        container.innerHTML = suggestionsHTML;
        container.style.display = 'block';
    }

    /**
     * Hide search suggestions
     */
    hideSuggestions() {
        const container = document.getElementById('search-suggestions');
        if (container) {
            container.style.display = 'none';
        }
    }

    /**
     * Display search results
     */
    displayResults(jobs, analytics, pagination) {
        const resultsContainer = document.getElementById('search-results');
        const countContainer = document.getElementById('results-count');
        
        if (!resultsContainer) return;

        // Update results count
        if (countContainer) {
            countContainer.textContent = `${analytics.total_results} jobs found`;
        }

        if (jobs.length === 0) {
            resultsContainer.innerHTML = `
                <div class="no-results">
                    <i class="fas fa-search text-6xl text-gray-300 mb-4"></i>
                    <h3 class="text-xl font-semibold text-gray-600 mb-2">No jobs found</h3>
                    <p class="text-gray-500">Try adjusting your search criteria or filters</p>
                    <div class="mt-4">
                        <button onclick="searchManager.clearFilters()" class="btn btn-primary">
                            Clear Filters
                        </button>
                    </div>
                </div>
            `;
            return;
        }

        const jobsHTML = jobs.map(scoredJob => this.createJobCard(scoredJob)).join('');
        resultsContainer.innerHTML = jobsHTML;
        
        this.updatePagination(pagination);
    }

    /**
     * Create a job card HTML
     */
    createJobCard(scoredJob) {
        const job = scoredJob.job;
        const salaryDisplay = this.formatSalary(job.salary_range_start, job.salary_range_end);
        const relevancePercent = Math.round(scoredJob.relevance_score * 100);
        const matchReasons = scoredJob.match_reasons.slice(0, 3); // Show top 3 reasons
        
        return `
            <div class="job-card glass-card" data-job-id="${job.id}">
                <div class="job-card-header">
                    <h3 class="job-title">${job.title}</h3>
                    <div class="job-meta">
                        <span class="company">${job.company}</span>
                        <span class="location">${job.location}</span>
                        ${job.is_urgent ? '<span class="urgent-badge">Urgent</span>' : ''}
                    </div>
                </div>
                
                <div class="job-card-body">
                    <p class="job-description">${this.truncateText(job.description, 150)}</p>
                    
                    <div class="job-details">
                        <div class="salary">${salaryDisplay}</div>
                        <div class="job-type">${job.job_type}</div>
                        ${scoredJob.distance_km ? `<div class="distance">${scoredJob.distance_km.toFixed(1)} km away</div>` : ''}
                    </div>
                    
                    ${matchReasons.length > 0 ? `
                        <div class="match-reasons">
                            <div class="relevance-score">
                                <span class="score">${relevancePercent}% match</span>
                            </div>
                            <div class="reasons">
                                ${matchReasons.map(reason => `<span class="reason-tag">${reason}</span>`).join('')}
                            </div>
                        </div>
                    ` : ''}
                </div>
                
                <div class="job-card-footer">
                    <div class="job-actions">
                        <button class="btn btn-outline btn-sm" onclick="searchManager.saveJob('${job.id}')">
                            <i class="fas fa-bookmark"></i> Save
                        </button>
                        <button class="btn btn-primary btn-sm" onclick="searchManager.viewJob('${job.id}')">
                            View Details
                        </button>
                        <button class="btn btn-success btn-sm" onclick="searchManager.applyToJob('${job.id}')">
                            <i class="fas fa-paper-plane"></i> Apply
                        </button>
                    </div>
                    <div class="job-posted">
                        Posted ${this.formatRelativeTime(job.created_at)}
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * Update search analytics display
     */
    updateSearchAnalytics(analytics) {
        const analyticsContainer = document.getElementById('search-analytics');
        if (!analyticsContainer) return;

        const avgSalary = analytics.avg_salary ? `$${Math.round(analytics.avg_salary).toLocaleString()}` : 'N/A';
        
        analyticsContainer.innerHTML = `
            <div class="analytics-grid">
                <div class="analytics-item">
                    <div class="analytics-value">${analytics.total_results}</div>
                    <div class="analytics-label">Total Jobs</div>
                </div>
                <div class="analytics-item">
                    <div class="analytics-value">${avgSalary}</div>
                    <div class="analytics-label">Avg Salary</div>
                </div>
                <div class="analytics-item">
                    <div class="analytics-value">${analytics.top_employers.length}</div>
                    <div class="analytics-label">Employers</div>
                </div>
            </div>
            
            <div class="analytics-distributions">
                ${this.createDistributionChart('Salary Ranges', analytics.salary_distribution)}
                ${this.createDistributionChart('Locations', analytics.location_distribution)}
            </div>
        `;
    }

    /**
     * Create distribution chart
     */
    createDistributionChart(title, distribution) {
        const entries = Object.entries(distribution);
        if (entries.length === 0) return '';

        const maxValue = Math.max(...entries.map(([_, count]) => count));
        
        const barsHTML = entries.map(([label, count]) => {
            const percentage = (count / maxValue) * 100;
            return `
                <div class="chart-bar">
                    <div class="chart-label">${label}</div>
                    <div class="chart-bar-container">
                        <div class="chart-bar-fill" style="width: ${percentage}%"></div>
                        <span class="chart-value">${count}</span>
                    </div>
                </div>
            `;
        }).join('');

        return `
            <div class="distribution-chart">
                <h4 class="chart-title">${title}</h4>
                <div class="chart-bars">
                    ${barsHTML}
                </div>
            </div>
        `;
    }

    /**
     * Utility functions
     */
    debounceSearch() {
        if (this.searchTimeout) {
            clearTimeout(this.searchTimeout);
        }
        this.searchTimeout = setTimeout(() => this.performSearch(), 500);
    }

    clearSearch() {
        document.getElementById('search-input').value = '';
        this.searchState.query = '';
        this.performSearch();
    }

    clearFilters() {
        // Reset all filters
        this.searchState.filters = {
            jobTypes: [],
            locations: [],
            salaryMin: null,
            salaryMax: null,
            isUrgent: false,
            remotePossible: false,
            radiusKm: 50,
            latitude: null,
            longitude: null
        };
        
        // Reset UI elements
        document.querySelectorAll('input[type="checkbox"]').forEach(cb => cb.checked = false);
        document.querySelectorAll('input[type="range"]').forEach(slider => {
            slider.value = slider.defaultValue;
        });
        
        this.performSearch();
    }

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

    formatRelativeTime(dateString) {
        const date = new Date(dateString);
        const now = new Date();
        const diffMs = now - date;
        const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));
        
        if (diffDays === 0) return 'Today';
        if (diffDays === 1) return 'Yesterday';
        if (diffDays < 7) return `${diffDays} days ago`;
        if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
        return `${Math.floor(diffDays / 30)} months ago`;
    }

    showLoading(show) {
        const loader = document.getElementById('search-loader');
        if (loader) {
            loader.style.display = show ? 'block' : 'none';
        }
    }

    showNotification(message, type = 'info') {
        // Simple notification system
        const notification = document.createElement('div');
        notification.className = `notification notification-${type}`;
        notification.textContent = message;
        
        document.body.appendChild(notification);
        
        setTimeout(() => {
            notification.remove();
        }, 3000);
    }

    displayError(message) {
        const resultsContainer = document.getElementById('search-results');
        if (resultsContainer) {
            resultsContainer.innerHTML = `
                <div class="error-state">
                    <i class="fas fa-exclamation-triangle text-6xl text-red-300 mb-4"></i>
                    <h3 class="text-xl font-semibold text-red-600 mb-2">Search Error</h3>
                    <p class="text-gray-600">${message}</p>
                    <button onclick="searchManager.performSearch()" class="btn btn-primary mt-4">
                        Try Again
                    </button>
                </div>
            `;
        }
    }

    // Save/Load functionality
    saveSearchToHistory() {
        const searchItem = {
            query: this.searchState.query,
            filters: { ...this.searchState.filters },
            timestamp: Date.now()
        };
        
        this.searchHistory.unshift(searchItem);
        this.searchHistory = this.searchHistory.slice(0, 10); // Keep last 10 searches
        localStorage.setItem('loco_search_history', JSON.stringify(this.searchHistory));
    }

    loadSearchHistory() {
        try {
            const saved = localStorage.getItem('loco_search_history');
            return saved ? JSON.parse(saved) : [];
        } catch {
            return [];
        }
    }

    // Job actions
    async saveJob(jobId) {
        try {
            // TODO: Implement job saving API call
            this.showNotification('Job saved successfully!', 'success');
        } catch (error) {
            this.showNotification('Failed to save job', 'error');
        }
    }

    viewJob(jobId) {
        window.location.href = `/job-detail.html?id=${jobId}`;
    }

    applyToJob(jobId) {
        window.location.href = `/job-detail.html?id=${jobId}&apply=true`;
    }

    // Load trending searches
    async loadTrendingSearches() {
        try {
            const response = await fetch('http://localhost:3070/api/v1/search/trending');
            const data = await response.json();
            
            const container = document.getElementById('trending-searches');
            if (container && data.trending) {
                const trendingHTML = data.trending.map(search => 
                    `<button class="trending-tag" onclick="searchManager.searchTrending('${search}')">${search}</button>`
                ).join('');
                container.innerHTML = trendingHTML;
            }
        } catch (error) {
            console.error('Failed to load trending searches:', error);
        }
    }

    searchTrending(query) {
        document.getElementById('search-input').value = query;
        this.searchState.query = query;
        this.performSearch();
    }

    updatePagination(pagination) {
        const container = document.getElementById('pagination');
        if (!container) return;

        const { page, total, limit } = pagination;
        const totalPages = Math.ceil(total / limit);
        
        if (totalPages <= 1) {
            container.style.display = 'none';
            return;
        }

        container.style.display = 'block';
        
        let paginationHTML = '';
        
        // Previous button
        if (page > 1) {
            paginationHTML += `<button class="page-btn" data-page="${page - 1}">Previous</button>`;
        }
        
        // Page numbers
        const startPage = Math.max(1, page - 2);
        const endPage = Math.min(totalPages, page + 2);
        
        for (let i = startPage; i <= endPage; i++) {
            const isActive = i === page ? 'active' : '';
            paginationHTML += `<button class="page-btn ${isActive}" data-page="${i}">${i}</button>`;
        }
        
        // Next button
        if (page < totalPages) {
            paginationHTML += `<button class="page-btn" data-page="${page + 1}">Next</button>`;
        }
        
        container.innerHTML = paginationHTML;
    }
}

// Initialize search manager when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.searchManager = new AdvancedSearchManager();
});