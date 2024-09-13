import json
from uuid import UUID
from datetime import datetime
from dataclasses import dataclass

@dataclass
class CreateQueuedJob:
    priority: float
    parent: UUID
    metadata: dict

@dataclass
class QueuedJob(CreateQueuedJob):
    id: UUID
    created_at: datetime

    @classmethod
    def from_dict(cls, data):
        return cls(
            id=data["id"],
            priority=data["priority"],
            parent=UUID(data["parent"]),
            metadata=data.get("metadata", {}),
            created_at=data["created_at"]
        )

    def to_dict(self):
        return dict(
            id=self.id,
            priority=self.priority,
            parent=str(self.parent),
            metadata=self.metadata,
            created_at=self.created_at
        )
    
    def to_json(self):
        return json.dumps(self.to_dict(), default=str)
