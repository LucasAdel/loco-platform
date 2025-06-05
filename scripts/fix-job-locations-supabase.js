#!/usr/bin/env node

/**
 * Script to fix job locations in Supabase database
 * Ensures each job is positioned in its actual suburb
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

// Adelaide suburbs with their correct coordinates
const ADELAIDE_SUBURBS = {
    // CBD and Inner
    'adelaide': [-34.9285, 138.6007],
    'adelaide cbd': [-34.9285, 138.6007],
    'north adelaide': [-34.9065, 138.5934],
    'kent town': [-34.9206, 138.6201],
    'hackney': [-34.9167, 138.6142],
    'college park': [-34.9167, 138.6083],
    'thorngate': [-34.9167, 138.5833],
    'medindie': [-34.9000, 138.6000],
    'gilberton': [-34.9000, 138.6083],
    'walkerville': [-34.8917, 138.6167],
    'vale park': [-34.8833, 138.6167],
    
    // Eastern Suburbs
    'norwood': [-34.9206, 138.6326],
    'kensington': [-34.9211, 138.6453],
    'marryatville': [-34.9250, 138.6450],
    'kensington park': [-34.9253, 138.6489],
    'leabrook': [-34.9361, 138.6456],
    'burnside': [-34.9397, 138.6444],
    'linden park': [-34.9456, 138.6406],
    'tusmore': [-34.9431, 138.6489],
    'glen osmond': [-34.9556, 138.6339],
    'mount osmond': [-34.9633, 138.6450],
    'waterfall gully': [-34.9706, 138.6778],
    'crafers': [-35.0047, 138.7089],
    'stirling': [-35.0006, 138.7158],
    'aldgate': [-35.0156, 138.7356],
    
    // Western Suburbs
    'thebarton': [-34.9167, 138.5700],
    'torrensville': [-34.9192, 138.5611],
    'mile end': [-34.9250, 138.5683],
    'hilton': [-34.9333, 138.5600],
    'cowandilla': [-34.9350, 138.5475],
    'brooklyn park': [-34.9283, 138.5433],
    'lockleys': [-34.9283, 138.5317],
    'west beach': [-34.9467, 138.5083],
    'henley beach': [-34.9167, 138.4933],
    'grange': [-34.9017, 138.4883],
    'tennyson': [-34.8833, 138.4817],
    'west lakes': [-34.8667, 138.4917],
    'seaton': [-34.8917, 138.5083],
    'findon': [-34.9033, 138.5317],
    'fulham': [-34.9167, 138.5150],
    'flinders park': [-34.9083, 138.5433],
    
    // Northern Suburbs
    'prospect': [-34.8833, 138.5950],
    'nailsworth': [-34.8833, 138.6083],
    'sefton park': [-34.8750, 138.6083],
    'blair athol': [-34.8633, 138.5983],
    'kilburn': [-34.8583, 138.5917],
    'enfield': [-34.8483, 138.6017],
    'clearview': [-34.8450, 138.6117],
    'northgate': [-34.8433, 138.6283],
    'klemzig': [-34.8667, 138.6333],
    'hillcrest': [-34.8500, 138.6500],
    'gilles plains': [-34.8467, 138.6617],
    'greenacres': [-34.8667, 138.6550],
    'hampstead gardens': [-34.8683, 138.6433],
    'manningham': [-34.8750, 138.6417],
    'mawson lakes': [-34.8111, 138.6111],
    'salisbury': [-34.7639, 138.6444],
    'parafield': [-34.7917, 138.6333],
    'modbury': [-34.8306, 138.6833],
    'tea tree gully': [-34.8250, 138.7000],
    'golden grove': [-34.7833, 138.7000],
    
    // Southern Suburbs
    'unley': [-34.9506, 138.6089],
    'parkside': [-34.9456, 138.6122],
    'fullarton': [-34.9511, 138.6289],
    'highgate': [-34.9606, 138.6189],
    'myrtle bank': [-34.9639, 138.6289],
    'urrbrae': [-34.9706, 138.6339],
    'springfield': [-34.9717, 138.6156],
    'clapham': [-34.9917, 138.6033],
    'colonel light gardens': [-34.9722, 138.5989],
    'cumberland park': [-34.9667, 138.5950],
    'daw park': [-34.9750, 138.5867],
    'melrose park': [-34.9833, 138.5767],
    'edwardstown': [-34.9817, 138.5717],
    'south plympton': [-34.9783, 138.5550],
    'glandore': [-34.9633, 138.5683],
    'kurralta park': [-34.9500, 138.5633],
    'netley': [-34.9467, 138.5500],
    'ascot park': [-34.9917, 138.5533],
    'park holme': [-34.9967, 138.5517],
    'marion': [-35.0117, 138.5450],
    'oaklands park': [-35.0167, 138.5450],
    'warradale': [-35.0067, 138.5350],
    'brighton': [-35.0167, 138.5150],
    'seacliff': [-35.0367, 138.5167],
    'marino': [-35.0467, 138.5117],
    'hallett cove': [-35.0794, 138.5106],
    'sheidow park': [-35.0667, 138.5217],
    'morphett vale': [-35.1333, 138.5333],
    'christies beach': [-35.1342, 138.4742],
    'noarlunga': [-35.1389, 138.4917],
    
    // Port Adelaide Region
    'port adelaide': [-34.8478, 138.5078],
    'alberton': [-34.8583, 138.5183],
    'queenstown': [-34.8550, 138.5133],
    'rosewater': [-34.8483, 138.5200],
    'pennington': [-34.8717, 138.5217],
    'ottoway': [-34.8350, 138.5317],
    'north haven': [-34.7917, 138.4933],
    'osborne': [-34.8083, 138.4833],
    'taperoo': [-34.8133, 138.4900],
    'largs bay': [-34.8250, 138.4833],
    'semaphore': [-34.8394, 138.4825],
    
    // Hills and Other
    'blackwood': [-35.0194, 138.6133],
    'belair': [-35.0019, 138.6256],
    'glenalta': [-35.0042, 138.6106],
    'eden hills': [-35.0033, 138.6050],
    'flagstaff hill': [-35.0483, 138.5717],
    'aberfoyle park': [-35.0617, 138.5967],
    'happy valley': [-35.0833, 138.5633],
    'mount barker': [-35.0667, 138.8667],
    
    // More Northern
    'elizabeth': [-34.7139, 138.6706],
    'elizabeth north': [-34.7000, 138.6833],
    'elizabeth south': [-34.7278, 138.6578],
    'gawler': [-34.5972, 138.7444],
};

// Helper function to get suburb coordinates
function getSuburbCoordinates(suburb) {
    if (!suburb) return null;
    
    const suburbLower = suburb.toLowerCase().trim();
    
    // Direct lookup
    if (ADELAIDE_SUBURBS[suburbLower]) {
        return ADELAIDE_SUBURBS[suburbLower];
    }
    
    // Try without common suffixes
    const withoutSuffix = suburbLower
        .replace(/ north$/, '')
        .replace(/ south$/, '')
        .replace(/ east$/, '')
        .replace(/ west$/, '')
        .replace(/ heights$/, '')
        .replace(/ park$/, '')
        .replace(/ gardens$/, '');
    
    if (ADELAIDE_SUBURBS[withoutSuffix]) {
        return ADELAIDE_SUBURBS[withoutSuffix];
    }
    
    // Fuzzy match - find closest match
    const suburbKeys = Object.keys(ADELAIDE_SUBURBS);
    for (const key of suburbKeys) {
        if (key.includes(withoutSuffix) || withoutSuffix.includes(key)) {
            return ADELAIDE_SUBURBS[key];
        }
    }
    
    return null;
}

// Check if coordinates are valid for Adelaide
function isValidAdelaideLocation(lat, lng) {
    return lat >= -35.35 && lat <= -34.60 && lng >= 138.45 && lng <= 138.85;
}

// Fix swapped coordinates
function fixSwappedCoordinates(lat, lng) {
    // Australian coordinates should have negative latitude and positive longitude > 100
    if (lat > 0 && lng < 0 && lng > -40) {
        return [lng, lat];
    }
    
    // If longitude is in latitude range for Adelaide
    if (lng >= -35.35 && lng <= -34.60 && lat >= 138.45 && lat <= 138.85) {
        return [lng, lat];
    }
    
    return [lat, lng];
}

async function fixJobLocations() {
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

        console.log(`📊 Found ${jobs.length} total jobs to check`);

        // Statistics
        let stats = {
            total: jobs.length,
            fixed: 0,
            missingCoords: 0,
            invalidCoords: 0,
            swappedCoords: 0,
            suburbFixed: 0,
            unchanged: 0,
            failed: 0
        };

        // Process each job
        for (const job of jobs) {
            let needsUpdate = false;
            let newLat = job.latitude;
            let newLng = job.longitude;
            let fixDescription = '';

            // Check if coordinates are missing
            if (!job.latitude || !job.longitude) {
                stats.missingCoords++;
                needsUpdate = true;
                fixDescription = 'Missing coordinates';
                
                // Try to get coordinates from suburb
                const coords = getSuburbCoordinates(job.suburb);
                if (coords) {
                    [newLat, newLng] = coords;
                    stats.suburbFixed++;
                    fixDescription += ` - Fixed using suburb "${job.suburb}"`;
                } else {
                    // Default to Adelaide CBD
                    [newLat, newLng] = ADELAIDE_SUBURBS['adelaide'];
                    fixDescription += ` - Defaulted to Adelaide CBD (suburb "${job.suburb}" not found)`;
                }
            } else {
                // Check if coordinates are valid
                let [lat, lng] = fixSwappedCoordinates(job.latitude, job.longitude);
                
                if (lat !== job.latitude || lng !== job.longitude) {
                    stats.swappedCoords++;
                    newLat = lat;
                    newLng = lng;
                    needsUpdate = true;
                    fixDescription = `Swapped coordinates: (${job.latitude}, ${job.longitude}) -> (${lat}, ${lng})`;
                } else if (!isValidAdelaideLocation(lat, lng)) {
                    stats.invalidCoords++;
                    needsUpdate = true;
                    fixDescription = `Invalid coordinates (${lat}, ${lng})`;
                    
                    // Try to fix using suburb
                    const coords = getSuburbCoordinates(job.suburb);
                    if (coords) {
                        [newLat, newLng] = coords;
                        stats.suburbFixed++;
                        fixDescription += ` - Fixed using suburb "${job.suburb}"`;
                    } else {
                        // Default to Adelaide CBD
                        [newLat, newLng] = ADELAIDE_SUBURBS['adelaide'];
                        fixDescription += ` - Defaulted to Adelaide CBD`;
                    }
                }
            }

            // Update if needed
            if (needsUpdate) {
                const { error: updateError } = await supabase
                    .from('jobs')
                    .update({ 
                        latitude: newLat, 
                        longitude: newLng,
                        updated_at: new Date().toISOString()
                    })
                    .eq('id', job.id);

                if (updateError) {
                    console.error(`❌ Error updating job ${job.id}:`, updateError);
                    stats.failed++;
                } else {
                    stats.fixed++;
                    console.log(`✅ Fixed job "${job.title}" in ${job.suburb}: ${fixDescription}`);
                }
            } else {
                stats.unchanged++;
            }
        }

        // Print summary
        console.log('\n📊 Location Fix Summary:');
        console.log(`Total jobs processed: ${stats.total}`);
        console.log(`✅ Fixed: ${stats.fixed}`);
        console.log(`   - Missing coordinates: ${stats.missingCoords}`);
        console.log(`   - Invalid coordinates: ${stats.invalidCoords}`);
        console.log(`   - Swapped coordinates: ${stats.swappedCoords}`);
        console.log(`   - Fixed using suburb: ${stats.suburbFixed}`);
        console.log(`⏭️  Unchanged (already valid): ${stats.unchanged}`);
        console.log(`❌ Failed to update: ${stats.failed}`);

        // Show suburb distribution
        console.log('\n📍 Jobs per suburb (top 20):');
        const suburbCounts = {};
        for (const job of jobs) {
            const suburb = job.suburb || 'Unknown';
            suburbCounts[suburb] = (suburbCounts[suburb] || 0) + 1;
        }
        
        const sortedSuburbs = Object.entries(suburbCounts)
            .sort((a, b) => b[1] - a[1])
            .slice(0, 20);
        
        for (const [suburb, count] of sortedSuburbs) {
            console.log(`   ${suburb}: ${count} jobs`);
        }

    } catch (error) {
        console.error('❌ Unexpected error:', error);
    }
}

// Run the script
console.log('🚀 Starting job location fix script...\n');
fixJobLocations()
    .then(() => {
        console.log('\n✅ Script completed successfully');
        process.exit(0);
    })
    .catch((error) => {
        console.error('\n❌ Script failed:', error);
        process.exit(1);
    });