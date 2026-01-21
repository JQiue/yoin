import "./App.css";

import type { Option } from "./index.d";
import { useState, useEffect, type JSX } from "preact/compat";

type Comment = {
  id: number;
  rid: number;
  nickname: string;
  link: string;
  content: string;
  page_page: string;
  email: string;
  created_at: string;
  updated_at: string;
  up_vote: number;
  down_vote: number;
  location: string;
};

export const Icon = ({ name }: { name: string }) => {
  const icons: Record<string, string> = {
    thumbsUp: "👍",
    thumbsDown: "👎",
    clock: "🕒",
    alert: "⚠",
    refresh: "↻",
    send: "➤",
    link: "🔗",
    close: "✕",
    replyTo: "➥",
    reply: "💬",
  };

  return (
    <span role="img" aria-label={name}>
      {icons[name] || ""}
    </span>
  );
};

interface AppProps {
  config: Option;
}

const App: (props: AppProps) => JSX.Element = (props) => {
  const [rawData, setRawData] = useState<Comment[]>([]);
  const [comments, setComments] = useState<Comment[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [totalCount, setTotalCount] = useState(0);

  // 从本地存储初始化
  const [formData, setFormData] = useState(() => {
    const saved = localStorage.getItem("comment_user_info");
    const draft = localStorage.getItem("comment_draft_main");
    const userInfo = saved
      ? JSON.parse(saved)
      : { nickname: "", email: "", link: "" };
    return { ...userInfo, content: draft || "" };
  });

  const [replyTo, setReplyTo] = useState(null);
  const [replyFormData, setReplyFormData] = useState(() => {
    const saved = localStorage.getItem("comment_user_info");
    const draft = localStorage.getItem("comment_draft_reply");
    const userInfo = saved ? JSON.parse(saved) : { nickname: "", email: "" };
    return { ...userInfo, content: draft || "" };
  });

  const [submitting, setSubmitting] = useState(false);
  const [submitStatus, setSubmitStatus] = useState({ type: "", msg: "" });

  const API_URL = "http://localhost:7410/api/comments";

  useEffect(() => {
    const userInfo = {
      nickname: formData.nickname,
      email: formData.email,
      link: formData.link,
    };
    localStorage.setItem("comment_user_info", JSON.stringify(userInfo));
    localStorage.setItem("comment_draft_main", formData.content);
  }, [formData]);

  useEffect(() => {
    localStorage.setItem("comment_draft_reply", replyFormData.content);
    if (replyFormData.nickname || replyFormData.email) {
      const userInfo = JSON.parse(
        localStorage.getItem("comment_user_info") || "{}",
      );
      localStorage.setItem(
        "comment_user_info",
        JSON.stringify({
          ...userInfo,
          nickname: replyFormData.nickname,
          email: replyFormData.email,
        }),
      );
    }
  }, [replyFormData]);

  const fetchComments = async () => {
    setLoading(true);
    try {
      const api = new URL(API_URL);
      api.searchParams.set("site_id", props.config.site_id.toString());
      const response = await fetch(api);
      const json = await response.json();
      if (json.code === 0 && Array.isArray(json.data)) {
        const data: Comment[] = json.data;
        setRawData(data);
        setTotalCount(data.length);
        const mainComments = data.filter((c) => c.rid === 0);
        const allReplies = data.filter((c) => c.rid !== 0);
        const findRootId: (comment: Comment, allData: Comment[]) => number = (
          comment,
          allData,
        ) => {
          if (comment.rid === 0) return comment.id;
          const parent = allData.find((d) => d.id === comment.rid);
          return parent ? findRootId(parent, allData) : comment.id;
        };
        const structured = mainComments.map((main) => ({
          ...main,
          replies: allReplies.filter((r) => findRootId(r, data) === main.id),
        }));
        setComments(structured);
      }
    } catch (err) {
      setError("连接失败");
    } finally {
      setLoading(false);
    }
  };

  const handleInputChange = (e, isReply = false) => {
    const { name, value } = e.target;
    if (isReply) setReplyFormData((prev) => ({ ...prev, [name]: value }));
    else setFormData((prev) => ({ ...prev, [name]: value }));
  };

  const handleSubmit = async (e, rid = 0) => {
    e.preventDefault();
    const currentFormData = rid === 0 ? formData : replyFormData;
    if (!currentFormData.content || !currentFormData.nickname) return;

    setSubmitting(true);
    setSubmitStatus({ type: "", msg: "" });

    try {
      const response = await fetch(API_URL, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          site_id: props.config.site_id,
          nickname: currentFormData.nickname,
          link: rid === 0 ? currentFormData.link : "",
          content: currentFormData.content,
          page_page: location.pathname,
          email: currentFormData.email,
          rid: rid,
        }),
      });

      const result = await response.json();

      // 核心处理逻辑：检查业务 code 是否为 0
      if (result.code === 0) {
        setSubmitStatus({ type: "success", msg: "发布成功" });
        if (rid === 0) {
          setFormData((prev) => ({ ...prev, content: "" }));
          localStorage.removeItem("comment_draft_main");
        } else {
          setReplyFormData((prev) => ({ ...prev, content: "" }));
          localStorage.removeItem("comment_draft_reply");
          setReplyTo(null);
        }
        fetchComments();
      } else {
        // 如果业务 code 不为 0，展示后端传回的 msg
        setSubmitStatus({ type: "error", msg: result.msg || "操作失败" });
      }
    } catch (err) {
      setSubmitStatus({ type: "error", msg: "网络错误，请稍后再试" });
    } finally {
      setSubmitting(false);
      // 成功或失败的提示保留 5 秒
      setTimeout(() => setSubmitStatus({ type: "", msg: "" }), 5000);
    }
  };

  useEffect(() => {
    fetchComments();
  }, []);

  const formatDate = (dateStr) => {
    try {
      return new Date(dateStr).toLocaleString("zh-CN", {
        month: "numeric",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      });
    } catch (e) {
      return dateStr;
    }
  };

  const getParentNickname = (rid) => {
    const parent = rawData.find((c) => c.id === rid);
    return parent ? parent.nickname : null;
  };

  return (
    <div className="min-h-screen bg-gray-50 py-6 px-4 font-sans text-gray-900">
      <div className="max-w-2xl mx-auto">
        {/* 发表主表单 */}
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-4 mb-6">
          <form onSubmit={(e) => handleSubmit(e, 0)} className="space-y-3">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-2">
              <input
                type="text"
                name="nickname"
                placeholder="昵称"
                value={formData.nickname}
                onChange={(e) => handleInputChange(e)}
                className="w-full px-3 py-1.5 bg-gray-50 border border-gray-200 rounded text-sm outline-none focus:border-blue-400"
                required
              />
              <input
                type="email"
                name="email"
                placeholder="邮箱"
                value={formData.email}
                onChange={(e) => handleInputChange(e)}
                className="w-full px-3 py-1.5 bg-gray-50 border border-gray-200 rounded text-sm outline-none focus:border-blue-400"
              />
              <input
                type="url"
                name="link"
                placeholder="网址"
                value={formData.link}
                onChange={(e) => handleInputChange(e)}
                className="w-full px-3 py-1.5 bg-gray-50 border border-gray-200 rounded text-sm outline-none focus:border-blue-400"
              />
            </div>
            <textarea
              name="content"
              rows="2"
              placeholder="撰写评论..."
              value={formData.content}
              onChange={(e) => handleInputChange(e)}
              className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded text-sm outline-none resize-none focus:border-blue-400"
              required
            ></textarea>
            <div className="flex justify-between items-center">
              <div className="flex items-center gap-2">
                {submitStatus.msg && (
                  <span
                    className={`text-xs font-medium flex items-center gap-1 ${submitStatus.type === "success" ? "text-green-600" : "text-red-500"}`}
                  >
                    {submitStatus.type === "error" && <Icon name="alert" />}
                    {submitStatus.msg}
                  </span>
                )}
              </div>
              <button
                type="submit"
                disabled={submitting}
                className="bg-blue-600 text-white px-4 py-1.5 rounded text-sm font-bold hover:bg-blue-700 disabled:bg-gray-300 transition-all flex items-center gap-2"
              >
                {submitting ? <Icon name="refresh" /> : <Icon name="send" />}{" "}
                发送
              </button>
            </div>
          </form>
        </div>

        {/* 评论列表 */}
        <div className="space-y-3">
          {comments.map((comment) => (
            <div
              key={comment.id}
              className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden"
            >
              <CommentCard
                comment={comment}
                formatDate={formatDate}
                onReplyClick={() => setReplyTo(comment)}
              />

              {comment.replies && comment.replies.length > 0 && (
                <div className="bg-gray-50 border-t border-gray-100 px-3 pb-1">
                  {comment.replies.map((reply) => (
                    <CommentCard
                      key={reply.id}
                      comment={reply}
                      isReply
                      formatDate={formatDate}
                      onReplyClick={() => setReplyTo(reply)}
                      parentNickname={getParentNickname(reply.rid)}
                    />
                  ))}
                </div>
              )}

              {replyTo &&
                (comment.id === replyTo.id ||
                  comment.replies.some((r) => r.id === replyTo.id)) && (
                  <div className="p-4 bg-blue-50 border-t border-blue-100">
                    <div className="flex items-center justify-between mb-2 text-xs font-bold text-blue-800">
                      <span>回复 @{replyTo.nickname}</span>
                      <button
                        onClick={() => setReplyTo(null)}
                        className="hover:bg-blue-100 p-0.5 rounded transition-colors"
                      >
                        <Icon name="close" />
                      </button>
                    </div>
                    <div className="grid grid-cols-2 gap-2 mb-2">
                      <input
                        type="text"
                        name="nickname"
                        placeholder="昵称"
                        value={replyFormData.nickname}
                        onChange={(e) => handleInputChange(e, true)}
                        className="px-2 py-1 border border-blue-200 rounded text-xs outline-none focus:border-blue-400"
                        required
                      />
                      <input
                        type="email"
                        name="email"
                        placeholder="邮箱"
                        value={replyFormData.email}
                        onChange={(e) => handleInputChange(e, true)}
                        className="px-2 py-1 border border-blue-200 rounded text-xs outline-none focus:border-blue-400"
                      />
                    </div>
                    <textarea
                      name="content"
                      rows="1"
                      placeholder="回复内容..."
                      value={replyFormData.content}
                      onChange={(e) => handleInputChange(e, true)}
                      className="w-full px-2 py-1.5 border border-blue-200 rounded text-xs outline-none resize-none mb-2 focus:border-blue-400"
                      required
                    ></textarea>
                    <div className="flex justify-between items-center">
                      <span
                        className={`text-[10px] ${submitStatus.type === "error" ? "text-red-500" : "text-blue-600"}`}
                      >
                        {submitStatus.msg}
                      </span>
                      <div className="flex gap-2">
                        <button
                          type="button"
                          onClick={() => setReplyTo(null)}
                          className="text-xs text-gray-500 hover:text-gray-700"
                        >
                          取消
                        </button>
                        <button
                          type="button"
                          onClick={(e) => handleSubmit(e, replyTo.id)}
                          disabled={submitting}
                          className="bg-blue-600 text-white px-3 py-1 rounded text-xs font-bold hover:bg-blue-700 transition-colors shadow-sm"
                        >
                          {submitting ? (
                            <Icon name="refresh" />
                          ) : (
                            <Icon name="send" />
                          )}
                          发送
                        </button>
                      </div>
                    </div>
                  </div>
                )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

const CommentCard = ({
  comment,
  isReply = false,
  formatDate,
  onReplyClick,
  parentNickname,
}) => (
  <div
    className={`${isReply ? "ml-8 py-2.5 border-b border-gray-100 last:border-0" : "p-4"}`}
  >
    <div className="flex items-start gap-2.5">
      <div
        className={`shrink-0 w-7 h-7 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-white font-bold text-xs ${isReply ? "bg-indigo-400" : "bg-blue-500 shadow-inner"}`}
      >
        {comment.nickname?.[0]}
      </div>
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-x-2 mb-0.5 text-xs">
          <span className="font-bold text-gray-900 tracking-tight">
            {comment.nickname}
          </span>
          {isReply && parentNickname && (
            <span className="text-gray-400 flex items-center gap-1">
              <Icon name="replyTo" />
              <span className="text-blue-500 font-semibold hover:underline cursor-default">
                {parentNickname}
              </span>
            </span>
          )}
          <span className="text-[10px] text-gray-500 flex items-center gap-1 ml-auto opacity-70">
            <Icon name="clock" />
            {formatDate(comment.updated_at)}
          </span>
        </div>
        <p className="text-gray-700 text-[13px] leading-snug whitespace-pre-wrap">
          {comment.content}
        </p>
        <div className="mt-1.5 flex items-center justify-between text-[10px] text-gray-400">
          <div className="flex gap-4">
            <button className="flex items-center gap-1 hover:text-blue-600 transition-colors">
              <Icon name="thumbsUp" />
              {comment.up_vote || 0}
            </button>
            <button className="flex items-center gap-1 hover:text-red-600 transition-colors">
              <Icon name="thumbsDown" />
              {comment.down_vote || 0}
            </button>
            <button
              onClick={onReplyClick}
              className="text-blue-500 font-bold hover:text-blue-700"
            >
              <Icon name="reply" />
            </button>
          </div>
          <div className="hidden sm:flex gap-2 opacity-30 font-mono text-[8px] uppercase">
            <span>{comment.location || "LOCAL"}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
);

export default App;
