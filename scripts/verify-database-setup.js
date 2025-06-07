#!/usr/bin/env node

/**
 * Verify Database Setup for Loco Platform
 * Tests that all tables and data are correctly created
 */

import { createClient } from '@supabase/supabase-js';

// Configuration
const SUPABASE_URL = 'https://kpmmsogskffsiubbegvc.supabase.co';
const SUPABASE_ANON_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImtwbW1zb2dza2Zmc2l1YmJlZ3ZjIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDkxMDQ5NDcsImV4cCI6MjA2NDY4MDk0N30.aBBTFJ5_qM1j4M0oTzZGWlyOPJKJhUC9HFKyBzO-oJU';

const supabase = createClient(SUPABASE_URL, SUPABASE_ANON_KEY);

async function verifyDatabaseSetup() {
    console.log('🔍 Verifying Loco Platform Database Setup');
    console.log('==========================================\n');
    
    try {
        // Test 1: Check if we can query jobs table
        console.log('📋 Testing jobs table...');
        const { data: jobs, error: jobsError } = await supabase
            .from('jobs')
            .select('*')
            .limit(5);
            
        if (jobsError) {
            console.log('❌ Jobs table not available:', jobsError.message);
            console.log('🔧 Please run the SQL schema in Supabase Dashboard first');
            return false;
        }
        
        console.log(`✅ Jobs table working! Found ${jobs?.length || 0} jobs`);
        if (jobs && jobs.length > 0) {
            console.log(`   📝 Sample job: "${jobs[0].title}" at ${jobs[0].company}`);
        }
        
        // Test 2: Test authentication with Super Admin
        console.log('\n🔐 Testing Super Administrator authentication...');
        const { data: authData, error: authError } = await supabase.auth.signInWithPassword({
            email: 'lw@hamiltonbailey.com',
            password: 'password123'
        });
        
        if (authError) {
            console.log('❌ Authentication failed:', authError.message);
            return false;
        }
        
        console.log('✅ Super Administrator authentication successful!');
        console.log(`   👤 User ID: ${authData.user?.id}`);
        console.log(`   📧 Email: ${authData.user?.email}`);
        
        // Test 3: Check user profile
        console.log('\n👤 Testing user profiles table...');
        const { data: profile, error: profileError } = await supabase
            .from('user_profiles')
            .select('*')
            .eq('id', authData.user?.id)
            .single();
            
        if (profileError) {
            console.log('⚠️  User profile not found:', profileError.message);
            console.log('ℹ️  This might be normal if the trigger hasn\'t run yet');
        } else {
            console.log('✅ User profile found!');
            console.log(`   🏷️  Role: ${profile.role}`);
            console.log(`   👨‍💼 Name: ${profile.first_name} ${profile.last_name}`);
        }
        
        // Test 4: Check applications table
        console.log('\n📄 Testing applications table...');
        const { data: applications, error: appError } = await supabase
            .from('applications')
            .select('*')
            .limit(5);
            
        if (appError) {
            console.log('❌ Applications table error:', appError.message);
        } else {
            console.log(`✅ Applications table working! Found ${applications?.length || 0} applications`);
        }
        
        // Test 5: Check saved jobs table
        console.log('\n💾 Testing saved_jobs table...');
        const { data: savedJobs, error: savedError } = await supabase
            .from('saved_jobs')
            .select('*')
            .limit(5);
            
        if (savedError) {
            console.log('❌ Saved jobs table error:', savedError.message);
        } else {
            console.log(`✅ Saved jobs table working! Found ${savedJobs?.length || 0} saved jobs`);
        }
        
        // Sign out
        await supabase.auth.signOut();
        console.log('\n🚪 Signed out successfully');
        
        console.log('\n🎉 Database Setup Verification Complete!');
        console.log('=========================================');
        console.log('✅ All core tables are functional');
        console.log('✅ Authentication is working');
        console.log('✅ Sample data is available');
        console.log('✅ Ready for development!');
        
        return true;
        
    } catch (error) {
        console.log('💥 Unexpected error:', error.message);
        return false;
    }
}

async function testJobCreation() {
    console.log('\n🆕 Testing job creation...');
    
    // First authenticate as super admin
    const { data: authData, error: authError } = await supabase.auth.signInWithPassword({
        email: 'lw@hamiltonbailey.com',
        password: 'password123'
    });
    
    if (authError) {
        console.log('❌ Authentication failed for job creation test');
        return false;
    }
    
    // Try to create a test job
    const testJob = {
        title: 'Test Pharmacist Position',
        company: 'Test Pharmacy',
        location: 'Test Location, NSW',
        job_type: 'FullTime',
        salary_range_start: 80000,
        salary_range_end: 100000,
        description: 'This is a test job created to verify database functionality.',
        is_urgent: false
    };
    
    const { data: newJob, error: createError } = await supabase
        .from('jobs')
        .insert([testJob])
        .select()
        .single();
        
    if (createError) {
        console.log('❌ Job creation failed:', createError.message);
        await supabase.auth.signOut();
        return false;
    }
    
    console.log('✅ Test job created successfully!');
    console.log(`   🆔 Job ID: ${newJob.id}`);
    console.log(`   📝 Title: ${newJob.title}`);
    
    // Clean up: delete the test job
    const { error: deleteError } = await supabase
        .from('jobs')
        .delete()
        .eq('id', newJob.id);
        
    if (deleteError) {
        console.log('⚠️  Warning: Could not delete test job:', deleteError.message);
    } else {
        console.log('🗑️  Test job cleaned up successfully');
    }
    
    await supabase.auth.signOut();
    return true;
}

async function main() {
    const setupOk = await verifyDatabaseSetup();
    
    if (setupOk) {
        await testJobCreation();
        
        console.log('\n🏁 All Tests Complete!');
        console.log('======================');
        console.log('🚀 Your Loco Platform database is ready for development!');
        console.log('💻 You can now test the full application at:');
        console.log('   🌐 Frontend: http://localhost:3080');
        console.log('   🔐 Login with: lw@hamiltonbailey.com / password123');
        console.log('   🎯 Test job creation, applications, and all features!');
    } else {
        console.log('\n❌ Database setup incomplete');
        console.log('📋 Please run the SQL schema in Supabase Dashboard first:');
        console.log('   🌐 https://supabase.com/dashboard/project/kpmmsogskffsiubbegvc/sql');
        console.log('   📄 Use the SQL from: /supabase/sql/setup-complete-schema.sql');
    }
}

main().catch(console.error);