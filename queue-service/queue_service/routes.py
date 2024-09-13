from flask import Blueprint, current_app, jsonify, request
from queue_service.models import CreateQueuedJob
from queue_service.utils import timed

import uuid
import json

queue_bp = Blueprint('queue', __name__)

@queue_bp.route('/push', methods=['PUT'])
def queue_push():
    logger = current_app.config['Logger']
    queue_service = current_app.config['QueueService']
    
    with timed('Push'):
        create_queued_job = CreateQueuedJob(priority=2, parent=uuid.uuid4(), metadata={})
        queued_job = queue_service.insert_queued_job(create_queued_job)

    return jsonify(queued_job.to_dict())

@queue_bp.route('/pop', methods=['DELETE'])
def queue_pop():
    batch_size = request.args.get('batch_size', 1)
    logger = current_app.config['Logger']
    queue_service = current_app.config['QueueService']
    
    with timed('Pop'):
        queued_jobs = queue_service.pop_queued_job(batch_size)

    return jsonify(queued_jobs) 