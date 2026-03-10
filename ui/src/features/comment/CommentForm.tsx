import type { TargetedEvent, TargetedSubmitEvent } from "preact";
import { useEffect, useState } from "preact/compat";
import { sendComment } from "../../api/comment";
import type { Comment } from "../../api/types";
import { storage } from "../../helper";
import { useAutoResizeTextarea } from "../../hook/useAutoResizeTextarea";
import { useStoredUser } from "../../hook/useStoredUser";
import { useCommentStore, useConfigStore, useCommentFormStore } from "../../store";
import Login from "../auth/Login";
import type { CommentForm } from "../../store/type";
import CommentIdentityBar from "./CommentIdentityBar";
import CommentComposer from "./CommentComposer";
import CommentSubmitStatus from "./CommentSubmitStatus";

interface Props {
	parent_id?: number;
	placeholder?: string;
	cb?: (createdComment?: Comment) => void;
}

export default (props: Props) => {
	const { config } = useConfigStore();
	const { nickname, email, content, website, setField } = useCommentFormStore();
	const { fetchComments } = useCommentStore();
	const [submitting, setSubmitting] = useState(false);
	const [submitStatus, setSubmitStatus] = useState({ type: "", msg: "" });
	const [isLoginModalOpen, setLoginModalOpen] = useState(false);
	const { currentUser, syncUserFromStorage, clearStoredUser } = useStoredUser();
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
		setLoginModalOpen(false);
	};

	const handleLogout = () => {
		clearStoredUser();
		setField("nickname", "");
		setField("email", "");
		setField("website", "");
	};

	const handleInputChange = (
		e: TargetedEvent<HTMLTextAreaElement | HTMLInputElement>,
	) => {
		const { name, value } = e.currentTarget;
		setField(name as keyof CommentForm, value);
		if (name === "content") {
			storage.set("yoin:comment_draft", value);
		}
	};

	const handleSubmit = async (e: TargetedSubmitEvent<HTMLFormElement>) => {
		e.preventDefault();
		setSubmitting(true);
		setSubmitStatus({ type: "", msg: "" });

		try {
			const resData = await sendComment(
				config.site_id,
				nickname,
				email,
				website,
				content,
				location.pathname,
				props.parent_id,
			);

			if (resData.code === 0) {
				setField("content", "");
				setSubmitStatus({ type: "success", msg: resData.msg });
				storage.set("yoin:user_info", {
					nickname,
					website,
					email,
					avatar: currentUser?.avatar,
				});
				storage.remove("yoin:comment_draft");
				fetchComments();
				props.cb?.(resData.data);
			}
		} catch (error: any) {
			setSubmitStatus({ type: "error", msg: error.toString() });
		} finally {
			setSubmitting(false);
		}
	};

	useEffect(() => {
		syncUserFromStorage();
	}, []);

	useEffect(() => {
		if (!isLoginModalOpen) return;

		const onKeyDown = (event: KeyboardEvent) => {
			if (event.key === "Escape") {
				setLoginModalOpen(false);
			}
		};

		document.body.style.overflow = "hidden";
		window.addEventListener("keydown", onKeyDown);

		return () => {
			document.body.style.overflow = "";
			window.removeEventListener("keydown", onKeyDown);
		};
	}, [isLoginModalOpen]);

	return (
		<div className="text-sm transition-all">
			<form className="space-y-3" onSubmit={(e) => handleSubmit(e)}>
				<CommentIdentityBar
					currentUser={currentUser}
					fields={fields}
					fieldValues={fieldValues}
					onInputChange={handleInputChange}
					onLoginClick={() => setLoginModalOpen(true)}
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

			{isLoginModalOpen && (
				<div
					className="fixed inset-0 z-50 flex items-center justify-center bg-zinc-400/40 backdrop-blur-sm"
					onClick={() => setLoginModalOpen(false)}
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
