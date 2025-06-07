#!/usr/bin/env node

/**
 * Create Super Administrator User for Loco Platform
 * This script creates the super admin user in Supabase Auth
 */

import { createClient } from '@supabase/supabase-js';

// Configuration from .env
const SUPABASE_URL = 'https://kpmmsogskffsiubbegvc.supabase.co';
const SUPABASE_SERVICE_ROLE_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImtwbW1zb2dza2Zmc2l1YmJlZ3ZjIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc0OTEwNDk0NywiZXhwIjoyMDY0NjgwOTQ3fQ.L3LvS0AbZoVGe0fHXUMYYi9I-M7Q64Rr8KnwfWL-25w';

// Super Administrator credentials
const SUPER_ADMIN_EMAIL = 'lw@hamiltonbailey.com';
const SUPER_ADMIN_PASSWORD = 'password123';

async function createSuperAdmin() {
    console.log('🔧 Creating Supabase client with service role...');
    
    // Create Supabase client with service role key for admin operations
    const supabase = createClient(SUPABASE_URL, SUPABASE_SERVICE_ROLE_KEY, {
        auth: {
            autoRefreshToken: false,
            persistSession: false
        }
    });

    try {
        console.log('👤 Creating Super Administrator user...');
        
        // Create the super admin user
        const { data: user, error: createError } = await supabase.auth.admin.createUser({
            email: SUPER_ADMIN_EMAIL,
            password: SUPER_ADMIN_PASSWORD,
            email_confirm: true, // Auto-confirm the email
            user_metadata: {
                firstName: 'Super',
                lastName: 'Administrator',
                role: 'super_admin'
            }
        });

        if (createError) {
            console.error('❌ Error creating user:', createError.message);
            
            // If user already exists, try to update instead
            if (createError.message.includes('already registered')) {
                console.log('🔄 User already exists, updating instead...');
                
                // Get the existing user
                const { data: existingUser, error: listError } = await supabase.auth.admin.listUsers();
                
                if (listError) {
                    console.error('❌ Error listing users:', listError.message);
                    return;
                }
                
                const targetUser = existingUser.users.find(u => u.email === SUPER_ADMIN_EMAIL);
                
                if (targetUser) {
                    console.log('📝 Updating existing user to Super Administrator...');
                    
                    const { data: updatedUser, error: updateError } = await supabase.auth.admin.updateUserById(
                        targetUser.id,
                        {
                            password: SUPER_ADMIN_PASSWORD,
                            email_confirm: true,
                            user_metadata: {
                                firstName: 'Super',
                                lastName: 'Administrator',
                                role: 'super_admin'
                            }
                        }
                    );
                    
                    if (updateError) {
                        console.error('❌ Error updating user:', updateError.message);
                        return;
                    }
                    
                    console.log('✅ Successfully updated Super Administrator!');
                    console.log('📧 Email:', SUPER_ADMIN_EMAIL);
                    console.log('🔑 Password:', SUPER_ADMIN_PASSWORD);
                    console.log('👤 User ID:', updatedUser.user.id);
                } else {
                    console.error('❌ Could not find existing user');
                }
            }
            return;
        }

        console.log('✅ Successfully created Super Administrator!');
        console.log('📧 Email:', SUPER_ADMIN_EMAIL);
        console.log('🔑 Password:', SUPER_ADMIN_PASSWORD);
        console.log('👤 User ID:', user.user.id);

        // Verify the user profile was created with the trigger
        console.log('🔍 Verifying user profile creation...');
        
        const { data: profile, error: profileError } = await supabase
            .from('user_profiles')
            .select('*')
            .eq('id', user.user.id)
            .single();

        if (profileError) {
            console.warn('⚠️ Warning: Could not verify profile creation:', profileError.message);
        } else {
            console.log('✅ User profile created with role:', profile.role);
        }

    } catch (error) {
        console.error('❌ Unexpected error:', error.message);
    }
}

async function testSuperAdminLogin() {
    console.log('\n🧪 Testing Super Administrator login...');
    
    // Create a new client for testing login
    const testClient = createClient(SUPABASE_URL, 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImtwbW1zb2dza2Zmc2l1YmJlZ3ZjIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDkxMDQ5NDcsImV4cCI6MjA2NDY4MDk0N30.SqVBXS-r8eG0jxo2lCdHiCKEiAHDpTJbKqfr0NGeSqM');
    
    try {
        const { data, error } = await testClient.auth.signInWithPassword({
            email: SUPER_ADMIN_EMAIL,
            password: SUPER_ADMIN_PASSWORD
        });

        if (error) {
            console.error('❌ Login test failed:', error.message);
            return;
        }

        console.log('✅ Login test successful!');
        console.log('🎫 Access token received');
        console.log('👤 User authenticated:', data.user.email);

        // Test accessing protected data
        const { data: profile, error: profileError } = await testClient
            .from('user_profiles')
            .select('*')
            .eq('id', data.user.id)
            .single();

        if (profileError) {
            console.warn('⚠️ Could not fetch user profile:', profileError.message);
        } else {
            console.log('✅ Profile access successful, role:', profile.role);
        }

        // Sign out
        await testClient.auth.signOut();
        console.log('🚪 Signed out successfully');

    } catch (error) {
        console.error('❌ Login test error:', error.message);
    }
}

// Main execution
async function main() {
    console.log('🚀 Setting up Super Administrator for Loco Platform\n');
    
    await createSuperAdmin();
    await testSuperAdminLogin();
    
    console.log('\n🎉 Super Administrator setup complete!');
    console.log('📝 You can now use these credentials in the login page:');
    console.log(`   Email: ${SUPER_ADMIN_EMAIL}`);
    console.log(`   Password: ${SUPER_ADMIN_PASSWORD}`);
}

// Run the script
main().catch(console.error);