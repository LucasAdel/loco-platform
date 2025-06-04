#!/usr/bin/env python3
"""
Simple Python backend for Loco Platform demo
Serves basic API endpoints while the Rust backend is being fixed
"""

from flask import Flask, jsonify, request
from flask_cors import CORS
import json
from datetime import datetime, timedelta
import uuid

app = Flask(__name__)
CORS(app)  # Enable CORS for all domains on all routes

# Sample Australian pharmacy jobs data
SAMPLE_JOBS = [
    {
        "id": str(uuid.uuid4()),
        "title": "Senior Pharmacist - Sydney CBD",
        "description": "Exciting opportunity for an experienced pharmacist to join our busy CBD pharmacy. AHPRA registration required.",
        "pharmacy_name": "City Health Pharmacy",
        "hourly_rate": 52.00,
        "address": "123 George Street, Sydney",
        "suburb": "Sydney",
        "postcode": "2000",
        "state": "NSW",
        "latitude": -33.8688,
        "longitude": 151.2093,
        "start_date": (datetime.now() + timedelta(days=7)).isoformat(),
        "end_date": (datetime.now() + timedelta(days=37)).isoformat(),
        "start_time": "08:30",
        "end_time": "17:30",
        "job_type": "Pharmacist",
        "status": "Active",
        "is_urgent": True,
        "distance_km": 2.5,
        "created_at": datetime.now().isoformat(),
        "updated_at": datetime.now().isoformat(),
        "created_by": str(uuid.uuid4())
    },
    {
        "id": str(uuid.uuid4()),
        "title": "Pharmacy Intern - Melbourne",
        "description": "12-month internship program for pharmacy graduates. Comprehensive training and mentorship provided.",
        "pharmacy_name": "Melbourne Central Pharmacy",
        "hourly_rate": 35.00,
        "address": "456 Collins Street, Melbourne",
        "suburb": "Melbourne",
        "postcode": "3000",
        "state": "VIC",
        "latitude": -37.8136,
        "longitude": 144.9631,
        "start_date": (datetime.now() + timedelta(days=14)).isoformat(),
        "end_date": (datetime.now() + timedelta(days=379)).isoformat(),
        "start_time": "09:00",
        "end_time": "17:00",
        "job_type": "Intern",
        "status": "Active",
        "is_urgent": False,
        "distance_km": 8.7,
        "created_at": datetime.now().isoformat(),
        "updated_at": datetime.now().isoformat(),
        "created_by": str(uuid.uuid4())
    },
    {
        "id": str(uuid.uuid4()),
        "title": "Part-time Pharmacy Assistant - Brisbane",
        "description": "Perfect for pharmacy students. Flexible hours, supportive team environment.",
        "pharmacy_name": "Sunshine Coast Pharmacy",
        "hourly_rate": 25.50,
        "address": "789 Queen Street, Brisbane",
        "suburb": "Brisbane",
        "postcode": "4000",
        "state": "QLD",
        "latitude": -27.4698,
        "longitude": 153.0251,
        "start_date": (datetime.now() + timedelta(days=3)).isoformat(),
        "end_date": (datetime.now() + timedelta(days=93)).isoformat(),
        "start_time": "10:00",
        "end_time": "15:00",
        "job_type": "PharmacyAssistant",
        "status": "Active",
        "is_urgent": False,
        "distance_km": 12.3,
        "created_at": datetime.now().isoformat(),
        "updated_at": datetime.now().isoformat(),
        "created_by": str(uuid.uuid4())
    },
    {
        "id": str(uuid.uuid4()),
        "title": "URGENT: Weekend Pharmacist - Perth",
        "description": "Weekend locum pharmacist needed immediately. Competitive rates for experienced professionals.",
        "pharmacy_name": "Perth Metro Pharmacy",
        "hourly_rate": 65.00,
        "address": "321 Hay Street, Perth",
        "suburb": "Perth",
        "postcode": "6000",
        "state": "WA",
        "latitude": -31.9505,
        "longitude": 115.8605,
        "start_date": (datetime.now() + timedelta(days=2)).isoformat(),
        "end_date": (datetime.now() + timedelta(days=16)).isoformat(),
        "start_time": "09:00",
        "end_time": "18:00",
        "job_type": "Pharmacist",
        "status": "Active",
        "is_urgent": True,
        "distance_km": 5.1,
        "created_at": datetime.now().isoformat(),
        "updated_at": datetime.now().isoformat(),
        "created_by": str(uuid.uuid4())
    }
]

@app.route('/')
def root():
    return jsonify({
        "message": "Loco Platform API - Python Demo Backend",
        "status": "running",
        "mode": "demo",
        "endpoints": [
            "/api/jobs",
            "/api/jobs/search",
            "/health"
        ]
    })

@app.route('/health')
def health():
    return jsonify({
        "status": "ok",
        "mode": "python_demo",
        "message": "Demo backend running successfully",
        "timestamp": datetime.now().isoformat()
    })

@app.route('/api/jobs', methods=['GET'])
def get_jobs():
    """Get all jobs with optional filtering"""
    # Get query parameters
    job_type = request.args.get('job_type')
    state = request.args.get('state')
    is_urgent = request.args.get('is_urgent')
    
    jobs = SAMPLE_JOBS.copy()
    
    # Apply filters
    if job_type:
        jobs = [job for job in jobs if job['job_type'].lower() == job_type.lower()]
    
    if state:
        jobs = [job for job in jobs if job['state'].upper() == state.upper()]
    
    if is_urgent is not None:
        urgent_filter = is_urgent.lower() == 'true'
        jobs = [job for job in jobs if job['is_urgent'] == urgent_filter]
    
    return jsonify({
        "jobs": jobs,
        "total": len(jobs),
        "message": "Demo job listings from Python backend"
    })

@app.route('/api/jobs/search', methods=['POST'])
def search_jobs():
    """Search jobs with more complex filtering"""
    data = request.get_json() or {}
    filters = data.get('filters', {})
    
    jobs = SAMPLE_JOBS.copy()
    
    # Apply filters from request body
    if filters.get('job_type'):
        jobs = [job for job in jobs if job['job_type'].lower() == filters['job_type'].lower()]
    
    if filters.get('state'):
        jobs = [job for job in jobs if job['state'].upper() == filters['state'].upper()]
    
    if filters.get('min_rate'):
        jobs = [job for job in jobs if job['hourly_rate'] >= filters['min_rate']]
    
    if filters.get('max_rate'):
        jobs = [job for job in jobs if job['hourly_rate'] <= filters['max_rate']]
    
    if filters.get('is_urgent') is not None:
        jobs = [job for job in jobs if job['is_urgent'] == filters['is_urgent']]
    
    return jsonify({
        "jobs": jobs,
        "total_count": len(jobs),
        "page": 1,
        "limit": len(jobs),
        "has_more": False,
        "message": "Search results from Python demo backend"
    })

@app.route('/api/jobs/<job_id>', methods=['GET'])
def get_job(job_id):
    """Get a specific job by ID"""
    job = next((job for job in SAMPLE_JOBS if job['id'] == job_id), None)
    
    if job:
        return jsonify(job)
    else:
        return jsonify({
            "error": "Job not found",
            "message": f"No job found with ID: {job_id}"
        }), 404

if __name__ == '__main__':
    print("🚀 Starting Loco Platform Python Demo Backend...")
    print("🔗 Frontend: http://localhost:8080")
    print("🔗 Backend API: http://localhost:3000")
    print("📊 Health Check: http://localhost:3000/health")
    print("💼 Jobs API: http://localhost:3000/api/jobs")
    print("\n✨ Demo mode - serving Australian pharmacy job listings")
    
    app.run(host='0.0.0.0', port=3000, debug=True)