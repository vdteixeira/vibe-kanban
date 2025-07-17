import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { KanbanCard } from '@/components/ui/shadcn-io/kanban';
import {
  MoreHorizontal,
  Trash2,
  Edit,
  Loader2,
  CheckCircle,
  XCircle,
} from 'lucide-react';
import type { TaskWithAttemptStatus } from 'shared/types';

type Task = TaskWithAttemptStatus;

interface TaskCardProps {
  task: Task;
  index: number;
  status: string;
  onEdit: (task: Task) => void;
  onDelete: (taskId: string) => void;
  onViewDetails: (task: Task) => void;
}

export function TaskCard({
  task,
  index,
  status,
  onEdit,
  onDelete,
  onViewDetails,
}: TaskCardProps) {
  return (
    <KanbanCard
      key={task.id}
      id={task.id}
      name={task.title}
      index={index}
      parent={status}
      onClick={() => onViewDetails(task)}
    >
      <div className="space-y-2">
        <div className="flex items-start justify-between">
          <div className="flex-1 pr-2">
            <h4 className="font-medium text-sm break-words">{task.title}</h4>
          </div>
          <div className="flex items-center space-x-1">
            {/* In Progress Spinner */}
            {task.has_in_progress_attempt && (
              <Loader2 className="h-3 w-3 animate-spin text-blue-500" />
            )}
            {/* Merged Indicator */}
            {task.has_merged_attempt && (
              <CheckCircle className="h-3 w-3 text-green-500" />
            )}
            {/* Failed Indicator */}
            {task.has_failed_attempt && !task.has_merged_attempt && (
              <XCircle className="h-3 w-3 text-red-500" />
            )}
            {/* Actions Menu */}
            <div
              onPointerDown={(e) => e.stopPropagation()}
              onMouseDown={(e) => e.stopPropagation()}
              onClick={(e) => e.stopPropagation()}
            >
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button
                    variant="ghost"
                    size="sm"
                    className="h-6 w-6 p-0 hover:bg-muted"
                  >
                    <MoreHorizontal className="h-3 w-3" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={() => onEdit(task)}>
                    <Edit className="h-4 w-4 mr-2" />
                    Edit
                  </DropdownMenuItem>
                  <DropdownMenuItem
                    onClick={() => onDelete(task.id)}
                    className="text-destructive"
                  >
                    <Trash2 className="h-4 w-4 mr-2" />
                    Delete
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </div>
        </div>
        {task.description && (
          <div>
            <p className="text-xs text-muted-foreground break-words">
              {task.description.length > 130
                ? `${task.description.substring(0, 130)}...`
                : task.description}
            </p>
          </div>
        )}
        {task.prp && (
          <div className="bg-purple-50 border border-purple-200 rounded-md p-2">
            <p className="text-xs font-medium text-purple-700 mb-1">PRP</p>
            <p className="text-xs text-purple-600 break-words">
              {task.prp.length > 100
                ? `${task.prp.substring(0, 100)}...`
                : task.prp}
            </p>
          </div>
        )}
      </div>
    </KanbanCard>
  );
}
