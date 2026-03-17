import CommentCard from "./CommentCard";
import type { Comment } from "@/shared/api/types";

interface Props {
	comments: Comment[];
	isReply: boolean;
	onDeleteComment?: (deletedComment: Comment) => void;
	onReplyCreated?: (createdComment: Comment) => void;
}

export default (props: Props) => {
	if (props.comments.length === 0) return null;
	return (
		<div>
			{props.comments.map((comment) => (
				<CommentCard
					key={comment.id}
					comment={comment}
					onDeleteComment={props.onDeleteComment}
					onReplyCreated={props.onReplyCreated}
				/>
			))}
		</div>
	);
};
