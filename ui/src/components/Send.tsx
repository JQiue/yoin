import type { TargetedEvent, TargetedSubmitEvent } from "preact";
import { useEffect, useState } from "preact/compat";
import { sendComment } from "../api/comment";
import { storage } from "../helper";
import { useCommentStore, useConfigStore, useCommentFormStore } from "../store";
import Icon from "./Icon";
import Login from "./Login";
import { useAuthForm } from "../hook/useAuthForm";
import type { CommentForm } from "../store/type";

interface Props {
	parent_id?: number;
	placeholder?: string;
	cb?: () => void;
}

export default (props: Props) => {
	const { config } = useConfigStore();
	const { nickname, email, content, website, setField } = useCommentFormStore();
	const { fetchComments } = useCommentStore();
	const [submitting, setSubmitting] = useState(false);
	const [submitStatus, setSubmitStatus] = useState({ type: "", msg: "" });
	const [isLoginModalOpen, setLoginModalOpen] = useState(false);
	const handleAuthSuccess = () => {
		setLoginModalOpen(false);
	};

	const fields: { name: keyof Omit<CommentForm, "content">, placeholder: string, type: string }[] = [
		{ name: "nickname", placeholder: "昵称 *", type: "text" },
		{ name: "email", placeholder: "邮箱", type: "email" },
		{ name: "website", placeholder: "网址", type: "url" },
	];

	const fieldValues = { nickname, email, website };

	const { user } = useAuthForm();

	useEffect(() => {
		if (user) {
			setLoginModalOpen(false);
		}
	}, [user]);

	const handleInputChange = (
		e: TargetedEvent<HTMLTextAreaElement | HTMLInputElement>,
		_isReply = false,
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
				setSubmitStatus({ type: "success", msg: "发送成功" });
				storage.set(
					"yoin:user_info",
					{
						nickname,
						website,
						email,
					}
				);
				setSubmitting(false);
				storage.remove("yoin:comment_draft");
				fetchComments();
				props.cb?.();
			}
		} catch (error: any) {
			setSubmitStatus({ type: "error", msg: error.toString() });
			setSubmitting(false);
		}
	};

	return (
		<div className="p-5 bg-white border rounded-md shadow-sm">
			<form onSubmit={(e) => handleSubmit(e)} className="space-y-4">
				{user ? "用户头像" : <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
					{fields.map((field) => (
						<input
							className="w-full px-3 py-2 border rounded-sm bg-zinc-50 text-sm outline-none transition-all focus:bg-white focus:ring-1 placeholder:text-zinc-400"
							key={field.name}
							type={
								field.type
							}
							name={field.name}
							placeholder={
								field.placeholder
							}
							onChange={handleInputChange}
							required
							value={fieldValues[field.name]}
						// value={field.valueKey === 'nickname' ? nickname : field.valueKey === 'email' ? email : website}
						/>
					))}
				</div>}

				<textarea
					name="content"
					rows={3}
					placeholder="撰写评论..."
					value={content}
					onChange={handleInputChange}
					className="w-full px-3 py-2 bg-zinc-50 border rounded-sm text-sm outline-none transition-all focus:bg-white focus:border-brand-black focus:ring-1 focus:ring-brand-black/5 placeholder:text-zinc-400"
					required
				></textarea>

				<div className="flex justify-between items-center pt-1">
					<div>
						<button className="w-20 px-3 py-2 flex items-center justify-center gap-2 shadow-sm rounded-sm bg-zinc-800 text-white text-xs font-bold hover:bg-zinc-600 disabled:bg-zinc-300 disabled:cursor-not-allowed transition-all active:scale-95" type="button" onClick={() => { setLoginModalOpen(true) }}>Login</button>
					</div>
					<div className="flex items-center gap-2">
						{submitStatus.msg && (
							<span
								className={`text-xs font-bold flex items-center gap-1.5 px-2 py-1 rounded-md ${submitStatus.type === "success"
									? "text-zinc-600 bg-zinc-100"
									: "text-red-600 bg-red-50"
									}`}
							>
								{submitStatus.type === "error" && <Icon name="alert" />}
								{submitStatus.msg}
							</span>
						)}
					</div>
					<div className="text-center flex items-center gap-2">
						<span className="text-sm text-zinc-400">
							{content.length}
						</span>
						<button
							type="submit"
							disabled={submitting}
							className="w-20 px-3 py-2 flex items-center justify-center gap-2 shadow-sm rounded-sm bg-zinc-800 text-white text-xs font-bold hover:bg-zinc-600 disabled:bg-zinc-300 disabled:cursor-not-allowed transition-all active:scale-95"
						>
							{submitting ? <Icon name="refresh" /> : <Icon name="send" />}
							{submitting ? "发送中" : "发送"}
						</button>
					</div>
				</div>
			</form>
			{isLoginModalOpen && <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm" onClick={() => { setLoginModalOpen(false) }}>
				<div className="w-full max-w-sm bg-white p-5 rounded-md shadow-sm mx-4" onClick={(e) => e.stopPropagation()} >
					<Login onSuccess={handleAuthSuccess}></Login>
				</div>
			</div>}
		</div>
	);
};
