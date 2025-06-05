#!/usr/bin/env node

/**
 * Script to assign random Adelaide suburbs to jobs and fix their locations
 * This ensures jobs are distributed across Adelaide metro area
 */

import { createClient } from '@supabase/supabase-js';
import * as dotenv from 'dotenv';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

// Load environment variables
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
dotenv.config({ path: join(__dirname, '..', '.env') });

// Initialize Supabase client
const supabaseUrl = process.env.VITE_SUPABASE_URL;
const supabaseServiceKey = process.env.SUPABASE_SERVICE_ROLE_KEY;

if (!supabaseUrl || !supabaseServiceKey) {
    console.error('❌ Missing Supabase configuration in .env file');
    process.exit(1);
}

const supabase = createClient(supabaseUrl, supabaseServiceKey, {
    auth: {
        autoRefreshToken: false,
        persistSession: false
    }
});

// Adelaide suburbs organized by region for balanced distribution
const ADELAIDE_REGIONS = {
    'CBD & Inner': [
        { name: 'Adelaide', coords: [-34.9285, 138.6007], postcode: '5000' },
        { name: 'North Adelaide', coords: [-34.9065, 138.5934], postcode: '5006' },
        { name: 'Kent Town', coords: [-34.9206, 138.6201], postcode: '5067' },
        { name: 'Parkside', coords: [-34.9456, 138.6122], postcode: '5063' },
        { name: 'Wayville', coords: [-34.9456, 138.5922], postcode: '5034' },
    ],
    'Eastern': [
        { name: 'Norwood', coords: [-34.9206, 138.6326], postcode: '5067' },
        { name: 'Burnside', coords: [-34.9397, 138.6444], postcode: '5066' },
        { name: 'Kensington', coords: [-34.9211, 138.6453], postcode: '5068' },
        { name: 'Magill', coords: [-34.9139, 138.6694], postcode: '5072' },
        { name: 'Tranmere', coords: [-34.9028, 138.6611], postcode: '5073' },
        { name: 'Paradise', coords: [-34.8917, 138.6667], postcode: '5075' },
        { name: 'Campbelltown', coords: [-34.8806, 138.6611], postcode: '5074' },
        { name: 'Glen Osmond', coords: [-34.9556, 138.6339], postcode: '5064' },
    ],
    'Western': [
        { name: 'Thebarton', coords: [-34.9167, 138.5700], postcode: '5031' },
        { name: 'Mile End', coords: [-34.9250, 138.5683], postcode: '5031' },
        { name: 'Henley Beach', coords: [-34.9167, 138.4933], postcode: '5022' },
        { name: 'Grange', coords: [-34.9017, 138.4883], postcode: '5022' },
        { name: 'West Lakes', coords: [-34.8667, 138.4917], postcode: '5021' },
        { name: 'Fulham', coords: [-34.9167, 138.5150], postcode: '5024' },
        { name: 'Lockleys', coords: [-34.9283, 138.5317], postcode: '5032' },
    ],
    'Northern': [
        { name: 'Prospect', coords: [-34.8833, 138.5950], postcode: '5082' },
        { name: 'Blair Athol', coords: [-34.8633, 138.5983], postcode: '5084' },
        { name: 'Enfield', coords: [-34.8483, 138.6017], postcode: '5085' },
        { name: 'Greenacres', coords: [-34.8667, 138.6550], postcode: '5086' },
        { name: 'Modbury', coords: [-34.8306, 138.6833], postcode: '5092' },
        { name: 'Tea Tree Gully', coords: [-34.8250, 138.7000], postcode: '5091' },
        { name: 'Golden Grove', coords: [-34.7833, 138.7000], postcode: '5125' },
        { name: 'Mawson Lakes', coords: [-34.8111, 138.6111], postcode: '5095' },
        { name: 'Salisbury', coords: [-34.7639, 138.6444], postcode: '5108' },
    ],
    'Southern': [
        { name: 'Unley', coords: [-34.9506, 138.6089], postcode: '5061' },
        { name: 'Goodwood', coords: [-34.9506, 138.5850], postcode: '5034' },
        { name: 'Colonel Light Gardens', coords: [-34.9722, 138.5989], postcode: '5041' },
        { name: 'Edwardstown', coords: [-34.9817, 138.5717], postcode: '5039' },
        { name: 'Marion', coords: [-35.0117, 138.5450], postcode: '5043' },
        { name: 'Brighton', coords: [-35.0167, 138.5150], postcode: '5048' },
        { name: 'Morphett Vale', coords: [-35.1333, 138.5333], postcode: '5162' },
        { name: 'Christies Beach', coords: [-35.1342, 138.4742], postcode: '5165' },
    ],
    'Port Adelaide': [
        { name: 'Port Adelaide', coords: [-34.8478, 138.5078], postcode: '5015' },
        { name: 'Semaphore', coords: [-34.8394, 138.4825], postcode: '5019' },
        { name: 'Largs Bay', coords: [-34.8250, 138.4833], postcode: '5016' },
        { name: 'Alberton', coords: [-34.8583, 138.5183], postcode: '5014' },
    ],
    'Hills': [
        { name: 'Blackwood', coords: [-35.0194, 138.6133], postcode: '5051' },
        { name: 'Belair', coords: [-35.0019, 138.6256], postcode: '5052' },
        { name: 'Stirling', coords: [-35.0006, 138.7158], postcode: '5152' },
        { name: 'Mount Barker', coords: [-35.0667, 138.8667], postcode: '5251' },
    ]
};

// Flatten all suburbs into a single array for easy random selection
const ALL_ADELAIDE_SUBURBS = Object.values(ADELAIDE_REGIONS).flat();

// Get a random suburb from the list
function getRandomSuburb() {
    return ALL_ADELAIDE_SUBURBS[Math.floor(Math.random() * ALL_ADELAIDE_SUBURBS.length)];
}

// Get a weighted random suburb (prefers inner suburbs)
function getWeightedRandomSuburb() {
    const weights = {
        'CBD & Inner': 3,
        'Eastern': 2.5,
        'Western': 2,
        'Northern': 2,
        'Southern': 2,
        'Port Adelaide': 1,
        'Hills': 0.5
    };
    
    // Create weighted array
    const weightedSuburbs = [];
    for (const [region, suburbs] of Object.entries(ADELAIDE_REGIONS)) {
        const weight = weights[region] || 1;
        const count = Math.floor(weight * 10);
        for (let i = 0; i < count; i++) {
            weightedSuburbs.push(...suburbs);
        }
    }
    
    return weightedSuburbs[Math.floor(Math.random() * weightedSuburbs.length)];
}

async function assignSuburbsAndFixLocations() {
    try {
        console.log('🔍 Fetching all jobs from Supabase...');
        
        // Get all jobs
        const { data: jobs, error: fetchError } = await supabase
            .from('jobs')
            .select('*')
            .order('created_at', { ascending: false });

        if (fetchError) {
            console.error('❌ Error fetching jobs:', fetchError);
            return;
        }

        if (!jobs || jobs.length === 0) {
            console.log('ℹ️  No jobs found in the database');
            return;
        }

        console.log(`📊 Found ${jobs.length} total jobs`);

        // Statistics
        let stats = {
            total: jobs.length,
            updated: 0,
            withSuburb: 0,
            withoutSuburb: 0,
            failed: 0
        };

        // Count existing suburbs
        const existingSuburbs = new Set();
        jobs.forEach(job => {
            if (job.suburb && job.suburb !== 'null' && job.suburb !== 'Unknown') {
                existingSuburbs.add(job.suburb);
                stats.withSuburb++;
            } else {
                stats.withoutSuburb++;
            }
        });

        console.log(`\n📍 Current status:`);
        console.log(`   Jobs with valid suburb: ${stats.withSuburb}`);
        console.log(`   Jobs without suburb: ${stats.withoutSuburb}`);
        console.log(`   Unique suburbs: ${existingSuburbs.size}`);

        // Process each job
        const updates = [];
        for (const job of jobs) {
            let needsUpdate = false;
            let updateData = {};
            
            // Check if suburb needs assignment
            if (!job.suburb || job.suburb === 'null' || job.suburb === 'Unknown' || job.suburb.trim() === '') {
                // Assign a weighted random suburb
                const newSuburb = getWeightedRandomSuburb();
                updateData.suburb = newSuburb.name;
                updateData.postcode = newSuburb.postcode;
                updateData.latitude = newSuburb.coords[0];
                updateData.longitude = newSuburb.coords[1];
                updateData.state = 'SA'; // South Australia
                needsUpdate = true;
                
                console.log(`📍 Assigning "${job.title}" to ${newSuburb.name}`);
            } else {
                // Job has a suburb, but check if it's a valid Adelaide suburb
                const validAdelaideSuburb = ALL_ADELAIDE_SUBURBS.find(s => 
                    s.name.toLowerCase() === job.suburb.toLowerCase()
                );
                
                if (validAdelaideSuburb) {
                    // Update coordinates to match the suburb
                    updateData.latitude = validAdelaideSuburb.coords[0];
                    updateData.longitude = validAdelaideSuburb.coords[1];
                    updateData.postcode = validAdelaideSuburb.postcode;
                    updateData.state = 'SA';
                    needsUpdate = true;
                    
                    console.log(`✅ Fixing coordinates for "${job.title}" in ${job.suburb}`);
                } else {
                    // Non-Adelaide suburb, reassign to random Adelaide suburb
                    const newSuburb = getWeightedRandomSuburb();
                    updateData.suburb = newSuburb.name;
                    updateData.postcode = newSuburb.postcode;
                    updateData.latitude = newSuburb.coords[0];
                    updateData.longitude = newSuburb.coords[1];
                    updateData.state = 'SA';
                    needsUpdate = true;
                    
                    console.log(`🔄 Reassigning "${job.title}" from ${job.suburb} to ${newSuburb.name}`);
                }
            }
            
            if (needsUpdate) {
                updateData.updated_at = new Date().toISOString();
                updates.push({ id: job.id, data: updateData });
            }
        }

        // Batch update jobs
        console.log(`\n🔄 Updating ${updates.length} jobs...`);
        
        for (const update of updates) {
            const { error: updateError } = await supabase
                .from('jobs')
                .update(update.data)
                .eq('id', update.id);

            if (updateError) {
                console.error(`❌ Error updating job ${update.id}:`, updateError);
                stats.failed++;
            } else {
                stats.updated++;
            }
        }

        // Get updated suburb distribution
        const { data: updatedJobs } = await supabase
            .from('jobs')
            .select('suburb')
            .order('suburb');

        const suburbCounts = {};
        if (updatedJobs) {
            updatedJobs.forEach(job => {
                const suburb = job.suburb || 'Unknown';
                suburbCounts[suburb] = (suburbCounts[suburb] || 0) + 1;
            });
        }

        // Print summary
        console.log('\n📊 Update Summary:');
        console.log(`Total jobs processed: ${stats.total}`);
        console.log(`✅ Successfully updated: ${stats.updated}`);
        console.log(`❌ Failed to update: ${stats.failed}`);
        
        console.log('\n📍 New suburb distribution:');
        const sortedSuburbs = Object.entries(suburbCounts)
            .sort((a, b) => b[1] - a[1])
            .slice(0, 30);
        
        for (const [suburb, count] of sortedSuburbs) {
            console.log(`   ${suburb}: ${count} jobs`);
        }
        
        // Show region distribution
        console.log('\n🗺️  Jobs by region:');
        const regionCounts = {};
        for (const [region, suburbs] of Object.entries(ADELAIDE_REGIONS)) {
            regionCounts[region] = 0;
            for (const suburb of suburbs) {
                regionCounts[region] += suburbCounts[suburb.name] || 0;
            }
        }
        
        for (const [region, count] of Object.entries(regionCounts)) {
            console.log(`   ${region}: ${count} jobs`);
        }

    } catch (error) {
        console.error('❌ Unexpected error:', error);
    }
}

// Run the script
console.log('🚀 Starting suburb assignment and location fix...\n');
assignSuburbsAndFixLocations()
    .then(() => {
        console.log('\n✅ Script completed successfully');
        process.exit(0);
    })
    .catch((error) => {
        console.error('\n❌ Script failed:', error);
        process.exit(1);
    });