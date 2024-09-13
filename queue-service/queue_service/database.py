from psycopg2.pool import SimpleConnectionPool
from psycopg2.extras import RealDictCursor
from contextlib import contextmanager
from logging import Logger
from uuid import UUID

import json

from queue_service.models import CreateQueuedJob, QueuedJob

class QueueService:
    def __init__(self, pool: SimpleConnectionPool, logger: Logger):
        self.pool = pool
        self.logger = logger

    @contextmanager
    def getcursor(self):
        conn = self.pool.getconn()
        try:
            yield conn.cursor(cursor_factory=RealDictCursor)
        finally:
            conn.commit()
            self.pool.putconn(conn)

    def insert_queued_job(self, data: CreateQueuedJob) -> QueuedJob:
        try:
            # with here will take care of put connection when its done
            with self.getcursor() as cur:
                cur.execute('INSERT INTO queue (priority, parent, metadata) VALUES (%s, %s, %s) RETURNING id;',
                            (data.priority, str(data.parent), json.dumps(data.metadata),))
                uid = cur.fetchone()['id']

                cur.execute(f'SELECT id, priority, parent, metadata, created_at FROM queue WHERE id = %s;',
                            (uid,))
                queued_job = QueuedJob.from_dict(cur.fetchone())

                return queued_job

        except Exception as e:
            print("error in executing with exception: ", e)

    def pop_queued_job(self, batch_size: int) -> list[QueuedJob]:
        try:
            # with here will take care of put connection when its done
            with self.getcursor() as cur:
                cur.execute('DELETE FROM queue WHERE id IN (SELECT id FROM queue ORDER BY priority, created_at LIMIT %s) RETURNING *',
                            (batch_size,))
                
                queued_jobs = cur.fetchall()
                return queued_jobs

        except Exception as e:
            print("error in executing with exception: ", e)
