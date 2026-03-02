import CommentCard from "./CommentCard";
import type { Comment } from "../api/types";

interface Props {
	comments: Comment[];
	isReply: boolean;
}

export default (props: Props) => {
	if (props.comments.length === 0) return null;
	return (
		<div
		// className={
		// 	props.isRoot
		// 		? "space-y-4"
		// 		: "ml-6 sm:ml-10 border-l-2 border-gray-100 pl-4 mt-2 mb-2"
		// }
		>
			{props.comments.map((comment) => (
				<CommentCard key={comment.id} comment={comment} />
			))}
		</div>
	);
};
