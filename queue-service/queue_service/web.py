from flask import Blueprint, current_app, render_template
from queue_service.utils import timed

web_bp = Blueprint('web', __name__)

@web_bp.route('/', methods=['GET'])
def index():
    queue_service = current_app.config['QueueService']
    
    with timed('List Queued Jobs'):
        queued_jobs = queue_service.list_queued_job()

    return render_template('queue/index.html', queued_jobs=queued_jobs)
