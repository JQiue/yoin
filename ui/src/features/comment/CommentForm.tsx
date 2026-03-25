import type { TargetedEvent } from "preact";
import { useEffect } from "preact/hooks";
import type { Comment } from "@/shared/api/types";
import { useAutoResizeTextarea } from "@/hook/useAutoResizeTextarea";
import { useCommentLoginModal } from "@/hook/useCommentLoginModal";
import { useCommentSubmit } from "@/hook/useCommentSubmit";
import { useStoredUser } from "@/hook/useStoredUser";
import Login from "@/features/auth/Login";
import type { CommentForm } from "@/store/types";
import CommentIdentityBar from "@/features/comment/CommentIdentityBar";
import CommentComposer from "@/features/comment/CommentComposer";
import CommentSubmitStatus from "@/features/comment/CommentSubmitStatus";

interface Props {
	parent_id?: number;
	placeholder?: string;
	cb?: (createdComment?: Comment) => void;
}

export default (props: Props) => {
	const { currentUser, syncUserFromStorage, clearStoredUser } = useStoredUser();
	const { isOpen, open, close } = useCommentLoginModal();
	const {
		content,
		nickname,
		email,
		website,
		submitting,
		submitStatus,
		setFormField,
		handleSubmit,
	} = useCommentSubmit({
		parentId: props.parent_id,
		currentUser,
		onCreated: props.cb,
	});
	const textareaRef = useAutoResizeTextarea(content);

	const fields: {
		name: keyof Omit<CommentForm, "content">;
		placeholder: string;
		type: string;
	}[] = [
			{ name: "nickname", placeholder: "nickname", type: "text" },
			{ name: "email", placeholder: "email", type: "email" },
			{ name: "website", placeholder: "website", type: "url" },
		];

	const fieldValues = { nickname, email, website };

	const handleAuthSuccess = () => {
		syncUserFromStorage();
		close();
	};

	const handleLogout = () => {
		clearStoredUser();
		setFormField("nickname", "");
		setFormField("email", "");
		setFormField("website", "");
	};

	const handleInputChange = (
		e: TargetedEvent<HTMLTextAreaElement | HTMLInputElement>,
	) => {
		const { name, value } = e.currentTarget;
		setFormField(name as keyof CommentForm, value);
	};

	useEffect(() => {
		syncUserFromStorage();
	}, [syncUserFromStorage]);

	return (
		<div className="text-sm transition-all">
			<form className="space-y-3" onSubmit={handleSubmit}>
				<CommentIdentityBar
					currentUser={currentUser}
					fields={fields}
					fieldValues={fieldValues}
					onInputChange={handleInputChange}
					onLoginClick={open}
					onLogout={handleLogout}
				/>
				<CommentComposer
					content={content}
					currentUser={currentUser}
					submitting={submitting}
					textareaRef={textareaRef}
					onInputChange={handleInputChange}
				/>
				<CommentSubmitStatus type={submitStatus.type} msg={submitStatus.msg} />
			</form>

			{isOpen && (
				<div
					className="fixed inset-0 z-50 flex items-center justify-center bg-zinc-400/40 backdrop-blur-sm"
					onClick={close}
				>
					<div
						className="w-full max-w-sm bg-zinc-50 p-3"
						onClick={(e) => e.stopPropagation()}
					>
						<Login onSuccess={handleAuthSuccess}></Login>
					</div>
				</div>
			)}
		</div>
	);
};
