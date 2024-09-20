from logging import getLogger
from flask import Flask
from psycopg2.pool import SimpleConnectionPool
from queue_service.database import QueueService

def create_app():
    """For to use dynamic environment"""
    app = Flask(__name__)
    # app.config.from_object(config[config_name])
    
    pool = SimpleConnectionPool( 
        3, 20, user='postgres', password='postgres', 
        host='localhost', port='5432', database='queue-service')
    
    logger = getLogger(__name__)

    app.config['DbPool'] = pool
    app.config['Logger'] = logger
    app.config['QueueService'] = QueueService(pool, logger)

    from queue_service.routes import queue_bp
    app.register_blueprint(queue_bp)

    from queue_service.web import web_bp
    app.register_blueprint(web_bp)

    return app
