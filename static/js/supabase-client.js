/**
 * Supabase Client Configuration for Loco Platform
 * Handles authentication and database operations
 */

// Supabase configuration from environment - Loco Platform Project
const SUPABASE_URL = 'https://kpmmsogskffsiubbegvc.supabase.co';
const SUPABASE_ANON_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImtwbW1zb2dza2Zmc2l1YmJlZ3ZjIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDkxMDQ5NDcsImV4cCI6MjA2NDY4MDk0N30.SqVBXS-r8eG0jxo2lCdHiCKEiAHDpTJbKqfr0NGeSqM';

// Import Supabase from CDN
const script = document.createElement('script');
script.src = 'https://cdn.jsdelivr.net/npm/@supabase/supabase-js@2.39.0/dist/umd/supabase.js';
document.head.appendChild(script);

let supabase = null;
let supabaseReady = false;
const supabaseReadyCallbacks = [];

// Function to wait for Supabase to be ready
function onSupabaseReady(callback) {
    if (supabaseReady && supabase) {
        callback();
    } else {
        supabaseReadyCallbacks.push(callback);
    }
}

// Initialize Supabase client when script loads
script.onload = function() {
    try {
        supabase = window.supabase.createClient(SUPABASE_URL, SUPABASE_ANON_KEY, {
            auth: {
                autoRefreshToken: true,
                persistSession: true,
                detectSessionInUrl: true
            }
        });
        
        console.log('✅ Supabase client initialized successfully');
        supabaseReady = true;
        
        // Execute all pending callbacks
        supabaseReadyCallbacks.forEach(callback => callback());
        supabaseReadyCallbacks.length = 0;
        
        // Check for existing session on page load
        checkAuthState();
        
        // Listen for auth state changes
        supabase.auth.onAuthStateChange((event, session) => {
            console.log('Auth state changed:', event, session?.user?.email);
            handleAuthStateChange(event, session);
        });
        
    } catch (error) {
        console.error('❌ Failed to initialize Supabase client:', error);
    }
};

// Authentication functions
const SupabaseAuth = {
    // Get current user session
    async getCurrentUser() {
        if (!supabase) {
            console.warn('Supabase client not yet initialized');
            return null;
        }
        try {
            const { data: { user }, error } = await supabase.auth.getUser();
            if (error) throw error;
            return user;
        } catch (error) {
            console.error('Error getting current user:', error);
            return null;
        }
    },

    // Sign up new user
    async signUp(email, password, userData = {}) {
        try {
            const { data, error } = await supabase.auth.signUp({
                email,
                password,
                options: {
                    data: userData
                }
            });
            
            if (error) throw error;
            
            console.log('✅ User signed up successfully');
            return { success: true, data };
        } catch (error) {
            console.error('❌ Sign up error:', error);
            return { success: false, error: error.message };
        }
    },

    // Sign in existing user
    async signIn(email, password) {
        try {
            const { data, error } = await supabase.auth.signInWithPassword({
                email,
                password
            });
            
            if (error) throw error;
            
            console.log('✅ User signed in successfully');
            return { success: true, data };
        } catch (error) {
            console.error('❌ Sign in error:', error);
            return { success: false, error: error.message };
        }
    },

    // Sign out user
    async signOut() {
        try {
            const { error } = await supabase.auth.signOut();
            if (error) throw error;
            
            console.log('✅ User signed out successfully');
            return { success: true };
        } catch (error) {
            console.error('❌ Sign out error:', error);
            return { success: false, error: error.message };
        }
    },

    // Reset password
    async resetPassword(email) {
        try {
            const { error } = await supabase.auth.resetPasswordForEmail(email, {
                redirectTo: `${window.location.origin}/reset-password.html`,
            });
            
            if (error) throw error;
            
            console.log('✅ Password reset email sent');
            return { success: true };
        } catch (error) {
            console.error('❌ Password reset error:', error);
            return { success: false, error: error.message };
        }
    },

    // Update user password
    async updatePassword(newPassword) {
        try {
            const { error } = await supabase.auth.updateUser({
                password: newPassword
            });
            
            if (error) throw error;
            
            console.log('✅ Password updated successfully');
            return { success: true };
        } catch (error) {
            console.error('❌ Password update error:', error);
            return { success: false, error: error.message };
        }
    },

    // Update user profile
    async updateProfile(updates) {
        try {
            const { error } = await supabase.auth.updateUser({
                data: updates
            });
            
            if (error) throw error;
            
            console.log('✅ Profile updated successfully');
            return { success: true };
        } catch (error) {
            console.error('❌ Profile update error:', error);
            return { success: false, error: error.message };
        }
    }
};

// Database functions
const SupabaseDB = {
    // Get jobs with filtering
    async getJobs(filters = {}) {
        if (!supabase) {
            console.warn('Supabase client not yet initialized');
            return { success: false, error: 'Supabase not initialized' };
        }
        try {
            let query = supabase.from('jobs').select('*');
            
            if (filters.location) {
                query = query.ilike('location', `%${filters.location}%`);
            }
            
            if (filters.job_type) {
                query = query.eq('job_type', filters.job_type);
            }
            
            if (filters.search) {
                query = query.or(`title.ilike.%${filters.search}%,description.ilike.%${filters.search}%`);
            }
            
            if (filters.min_salary) {
                query = query.gte('salary_range_start', filters.min_salary);
            }
            
            if (filters.max_salary) {
                query = query.lte('salary_range_end', filters.max_salary);
            }
            
            const { data, error } = await query
                .order('created_at', { ascending: false })
                .limit(filters.limit || 20)
                .range(((filters.page || 1) - 1) * (filters.limit || 20), (filters.page || 1) * (filters.limit || 20) - 1);
            
            if (error) throw error;
            
            return { success: true, data };
        } catch (error) {
            console.error('❌ Error fetching jobs:', error);
            return { success: false, error: error.message };
        }
    },

    // Get single job by ID
    async getJob(id) {
        try {
            const { data, error } = await supabase
                .from('jobs')
                .select('*')
                .eq('id', id)
                .single();
            
            if (error) throw error;
            
            return { success: true, data };
        } catch (error) {
            console.error('❌ Error fetching job:', error);
            return { success: false, error: error.message };
        }
    },

    // Create new job (requires authentication)
    async createJob(jobData) {
        try {
            const user = await SupabaseAuth.getCurrentUser();
            if (!user) {
                throw new Error('User must be authenticated to create jobs');
            }
            
            const { data, error } = await supabase
                .from('jobs')
                .insert([{
                    ...jobData,
                    user_id: user.id,
                    created_at: new Date().toISOString()
                }])
                .select()
                .single();
            
            if (error) throw error;
            
            console.log('✅ Job created successfully');
            return { success: true, data };
        } catch (error) {
            console.error('❌ Error creating job:', error);
            return { success: false, error: error.message };
        }
    },

    // Update job (requires authentication and ownership)
    async updateJob(id, updates) {
        try {
            const user = await SupabaseAuth.getCurrentUser();
            if (!user) {
                throw new Error('User must be authenticated to update jobs');
            }
            
            const { data, error } = await supabase
                .from('jobs')
                .update({
                    ...updates,
                    updated_at: new Date().toISOString()
                })
                .eq('id', id)
                .eq('user_id', user.id) // Ensure user owns the job
                .select()
                .single();
            
            if (error) throw error;
            
            console.log('✅ Job updated successfully');
            return { success: true, data };
        } catch (error) {
            console.error('❌ Error updating job:', error);
            return { success: false, error: error.message };
        }
    },

    // Get user applications
    async getUserApplications() {
        try {
            const user = await SupabaseAuth.getCurrentUser();
            if (!user) {
                throw new Error('User must be authenticated');
            }
            
            const { data, error } = await supabase
                .from('applications')
                .select(`
                    *,
                    jobs (
                        id,
                        title,
                        company,
                        location
                    )
                `)
                .eq('user_id', user.id)
                .order('created_at', { ascending: false });
            
            if (error) throw error;
            
            return { success: true, data };
        } catch (error) {
            console.error('❌ Error fetching applications:', error);
            return { success: false, error: error.message };
        }
    },

    // Apply for job
    async applyForJob(jobId, applicationData = {}) {
        try {
            const user = await SupabaseAuth.getCurrentUser();
            if (!user) {
                throw new Error('User must be authenticated to apply for jobs');
            }
            
            const { data, error } = await supabase
                .from('applications')
                .insert([{
                    job_id: jobId,
                    user_id: user.id,
                    ...applicationData,
                    created_at: new Date().toISOString()
                }])
                .select()
                .single();
            
            if (error) throw error;
            
            console.log('✅ Application submitted successfully');
            return { success: true, data };
        } catch (error) {
            console.error('❌ Error submitting application:', error);
            return { success: false, error: error.message };
        }
    }
};

// Auth state management
function handleAuthStateChange(event, session) {
    const authElements = {
        loginButtons: document.querySelectorAll('.auth-login'),
        logoutButtons: document.querySelectorAll('.auth-logout'),
        userInfo: document.querySelectorAll('.user-info'),
        protectedContent: document.querySelectorAll('.protected-content'),
        publicContent: document.querySelectorAll('.public-content')
    };
    
    if (session?.user) {
        // User is logged in
        authElements.loginButtons.forEach(el => el.style.display = 'none');
        authElements.logoutButtons.forEach(el => el.style.display = 'block');
        authElements.protectedContent.forEach(el => el.style.display = 'block');
        authElements.publicContent.forEach(el => el.style.display = 'none');
        
        // Update user info displays
        authElements.userInfo.forEach(el => {
            el.textContent = session.user.email;
            el.style.display = 'block';
        });
        
        // Store user data
        localStorage.setItem('user_email', session.user.email);
        localStorage.setItem('user_id', session.user.id);
        
    } else {
        // User is logged out
        authElements.loginButtons.forEach(el => el.style.display = 'block');
        authElements.logoutButtons.forEach(el => el.style.display = 'none');
        authElements.protectedContent.forEach(el => el.style.display = 'none');
        authElements.publicContent.forEach(el => el.style.display = 'block');
        authElements.userInfo.forEach(el => el.style.display = 'none');
        
        // Clear stored user data
        localStorage.removeItem('user_email');
        localStorage.removeItem('user_id');
    }
}

// Check auth state on page load
async function checkAuthState() {
    try {
        const { data: { session }, error } = await supabase.auth.getSession();
        if (error) throw error;
        
        handleAuthStateChange('INITIAL_SESSION', session);
    } catch (error) {
        console.error('❌ Error checking auth state:', error);
    }
}

// Utility functions
const SupabaseUtils = {
    // Check if user is authenticated
    async isAuthenticated() {
        const user = await SupabaseAuth.getCurrentUser();
        return !!user;
    },

    // Require authentication (redirect to login if not authenticated)
    async requireAuth(redirectUrl = '/login.html') {
        const isAuth = await this.isAuthenticated();
        if (!isAuth) {
            window.location.href = redirectUrl;
            return false;
        }
        return true;
    },

    // Show loading state
    showLoading(elementId) {
        const element = document.getElementById(elementId);
        if (element) {
            element.innerHTML = '<div class="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600 mx-auto"></div>';
        }
    },

    // Show error message as bottom-right toast
    showError(message, elementId = null) {
        if (elementId) {
            const errorHtml = `
                <div class="bg-red-50 border border-red-200 rounded-lg p-4 text-red-700">
                    <div class="flex">
                        <svg class="w-5 h-5 mr-3 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
                        </svg>
                        <span>${message}</span>
                    </div>
                </div>
            `;
            const element = document.getElementById(elementId);
            if (element) {
                element.innerHTML = errorHtml;
            }
        } else {
            // Show as bottom-right toast
            this.showToast(message, 'error');
        }
    },

    // Show success message as bottom-right toast
    showSuccess(message, elementId = null) {
        if (elementId) {
            const successHtml = `
                <div class="bg-green-50 border border-green-200 rounded-lg p-4 text-green-700">
                    <div class="flex">
                        <svg class="w-5 h-5 mr-3 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                        </svg>
                        <span>${message}</span>
                    </div>
                </div>
            `;
            const element = document.getElementById(elementId);
            if (element) {
                element.innerHTML = successHtml;
            }
        } else {
            // Show as bottom-right toast
            this.showToast(message, 'success');
        }
    },

    // Universal toast notification system - appears from bottom right
    showToast(message, type = 'info', duration = 4000) {
        // Remove any existing toasts with same message to prevent duplicates
        const existingToasts = document.querySelectorAll('.toast-notification');
        existingToasts.forEach(toast => {
            if (toast.textContent.includes(message)) {
                toast.remove();
            }
        });

        const toast = document.createElement('div');
        toast.className = `toast-notification fixed bottom-4 right-4 z-50 max-w-sm transform translate-x-full transition-transform duration-300 ease-in-out`;
        
        let bgColor = 'bg-blue-500';
        let icon = '💡';
        
        switch (type) {
            case 'success':
                bgColor = 'bg-green-500';
                icon = '✅';
                break;
            case 'warning':
                bgColor = 'bg-yellow-500';
                icon = '⚠️';
                break;
            case 'error':
                bgColor = 'bg-red-500';
                icon = '❌';
                break;
            case 'info':
            default:
                bgColor = 'bg-blue-500';
                icon = '💡';
        }
        
        toast.innerHTML = `
            <div class="${bgColor} text-white px-4 py-3 rounded-lg shadow-lg flex items-center min-w-[300px]">
                <span class="mr-2 text-lg">${icon}</span>
                <span class="text-sm flex-1">${message}</span>
                <button onclick="this.parentElement.parentElement.remove()" class="ml-3 text-white hover:text-gray-200 flex-shrink-0">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                    </svg>
                </button>
            </div>
        `;
        
        document.body.appendChild(toast);
        
        // Animate in from right
        setTimeout(() => {
            toast.classList.remove('translate-x-full');
        }, 100);
        
        // Auto-remove after duration
        setTimeout(() => {
            if (document.body.contains(toast)) {
                toast.classList.add('translate-x-full');
                setTimeout(() => {
                    if (document.body.contains(toast)) {
                        document.body.removeChild(toast);
                    }
                }, 300);
            }
        }, duration);
    }
};

// Export for global use
window.SupabaseAuth = SupabaseAuth;
window.SupabaseDB = SupabaseDB;
window.SupabaseUtils = SupabaseUtils;
window.supabaseClient = supabase;

console.log('🔧 Supabase client configuration loaded');