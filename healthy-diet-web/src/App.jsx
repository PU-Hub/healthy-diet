import { useState, useEffect, useRef } from 'react';
import {
  Activity, User, MessageSquare, Camera, LogOut,
  Home, Edit2, Send, UploadCloud, CheckCircle2,
  Sparkles, Users, BookOpen, Smartphone, Trophy, Moon, Code2, Server, Database, X, Image as ImageIcon
} from 'lucide-react';
import ReactMarkdown from 'react-markdown';


const API_BASE = '/api';



const LoginView = ({ apiFetch, setToken, setCurrentView, showNotification }) => {
  const [email, setEmail] = useState('ckck@gmail.com');
  const [password, setPassword] = useState('a123456');
  const [isLoading, setIsLoading] = useState(false);

  const handleLogin = async (e) => {
    e.preventDefault();
    setIsLoading(true);
    try {
      const data = await apiFetch('/auth/login', {
        method: 'POST',
        body: JSON.stringify({ email, password })
      });
      setToken(data.token);
      localStorage.setItem('token', data.token);
      showNotification('登入成功！');
    } catch (err) {
      showNotification(err.message, 'error');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="bg-white p-6 sm:p-8 rounded-3xl shadow-2xl w-full max-w-md transform transition-all hover:scale-[1.01] mx-4">
      <div className="flex justify-center mb-6">
        <div className="`bg-linear-to-br from-green-400 to-emerald-600 p-4 rounded-2xl text-white shadow-lg">
          <Activity size={36} />
        </div>
      </div>
      <h2 className="text-2xl sm:text-3xl font-extrabold text-center text-gray-800 mb-2">健康飲食管家</h2>
      <p className="text-center text-gray-500 mb-8 text-sm sm:text-base">登入以繼續您的健康旅程</p>

      <form onSubmit={handleLogin} className="space-y-4 sm:space-y-5">
        <div>
          <label className="block text-sm font-semibold text-gray-700 mb-1">電子郵件</label>
          <input type="email" value={email} onChange={e => setEmail(e.target.value)} required
            className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 outline-none transition-all bg-gray-50 focus:bg-white text-sm sm:text-base"
            placeholder="請輸入信箱" />
        </div>
        <div>
          <label className="block text-sm font-semibold text-gray-700 mb-1">密碼</label>
          <input type="password" value={password} onChange={e => setPassword(e.target.value)} required
            className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 outline-none transition-all bg-gray-50 focus:bg-white text-sm sm:text-base"
            placeholder="請輸入密碼" />
        </div>
        <button type="submit" disabled={isLoading}
          className="w-full bg-linear-to-r from-emerald-500 to-green-600 text-white py-3 rounded-xl hover:from-emerald-600 hover:to-green-700 transition-all font-bold shadow-md disabled:opacity-70 mt-2">
          {isLoading ? '驗證中...' : '立即登入'}
        </button>
        <div className="text-center mt-6 pt-4 border-t border-gray-100">
          <span className="text-sm text-gray-600">還沒有專屬帳號嗎？</span>
          <button type="button" onClick={() => setCurrentView('register')} className="text-sm font-bold text-emerald-600 ml-1 hover:text-emerald-800 hover:underline transition-colors">
            免費註冊
          </button>
        </div>
      </form>
    </div>
  );
};

const RegisterView = ({ apiFetch, setToken, setCurrentView, showNotification }) => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [nickname, setNickname] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const handleRegister = async (e) => {
    e.preventDefault();
    setIsLoading(true);
    try {
      const data = await apiFetch('/auth/register', {
        method: 'POST',
        body: JSON.stringify({ email, password, nickname })
      });
      setToken(data.token);
      localStorage.setItem('token', data.token);
      showNotification('註冊成功！歡迎加入！');
    } catch (err) {
      showNotification(err.message, 'error');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="bg-white p-6 sm:p-8 rounded-3xl shadow-2xl w-full max-w-md transform transition-all hover:scale-[1.01] mx-4">
      <div className="flex justify-center mb-6">
        <div className="bg-linear-to-br from-blue-400 to-indigo-600 p-4 rounded-2xl text-white shadow-lg">
          <User size={36} />
        </div>
      </div>
      <h2 className="text-2xl sm:text-3xl font-extrabold text-center text-gray-800 mb-2">建立新帳號</h2>
      <p className="text-center text-gray-500 mb-8 text-sm sm:text-base">只需幾秒鐘，開啟健康新生活</p>

      <form onSubmit={handleRegister} className="space-y-4">
        <div>
          <label className="block text-sm font-semibold text-gray-700 mb-1">電子郵件</label>
          <input type="email" value={email} onChange={e => setEmail(e.target.value)} required
            className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 outline-none transition-all bg-gray-50 focus:bg-white text-sm sm:text-base" />
        </div>
        <div>
          <label className="block text-sm font-semibold text-gray-700 mb-1">密碼</label>
          <input type="password" value={password} onChange={e => setPassword(e.target.value)} required
            className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 outline-none transition-all bg-gray-50 focus:bg-white text-sm sm:text-base" />
        </div>
        <div>
          <label className="block text-sm font-semibold text-gray-700 mb-1">怎麼稱呼您？ (暱稱選填)</label>
          <input type="text" value={nickname} onChange={e => setNickname(e.target.value)}
            className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 outline-none transition-all bg-gray-50 focus:bg-white text-sm sm:text-base" />
        </div>
        <button type="submit" disabled={isLoading}
          className="w-full bg-linear-to-r from-indigo-500 to-blue-600 text-white py-3 rounded-xl hover:from-indigo-600 hover:to-blue-700 transition-all font-bold shadow-md disabled:opacity-70 mt-4">
          {isLoading ? '建立中...' : '完成註冊'}
        </button>
        <div className="text-center mt-6 pt-4 border-t border-gray-100">
          <span className="text-sm text-gray-600">已經有帳號了？</span>
          <button type="button" onClick={() => setCurrentView('login')} className="text-sm font-bold text-indigo-600 ml-1 hover:text-indigo-800 hover:underline transition-colors">
            返回登入
          </button>
        </div>
      </form>
    </div>
  );
};

// ==========================================
// 2. 豐富化的首頁 (Dashboard)
// ==========================================

const DashboardView = ({ user, setCurrentView }) => {
  const [health, setHealth] = useState(null);

  useEffect(() => {
    fetch(`${API_BASE}/health`)
      .then(() => setHealth({
        status: 'ok'
      }))
      .catch(console.error);
  }, []);

  const features = [
    { id: 'consult', name: 'AI 營養師', icon: <MessageSquare size={28} />, desc: '隨時為您解答飲食疑問', color: 'bg-emerald-50 text-emerald-600', hover: 'hover:bg-emerald-100 border-emerald-100' },
    { id: 'diet', name: '相機辨識', icon: <Camera size={28} />, desc: '拍下食物自動計算熱量', color: 'bg-blue-50 text-blue-600', hover: 'hover:bg-blue-100 border-blue-100' },
    { id: 'profile', name: '身體數據', icon: <Activity size={28} />, desc: '管理您的身高體重與目標', color: 'bg-purple-50 text-purple-600', hover: 'hover:bg-purple-100 border-purple-100' },
  ];

  const upcomingFeatures = [
    { title: '穿戴裝置串接', icon: <Smartphone size={24} />, desc: '支援 Apple Health & Google Fit 資料同步，精準計算每日消耗。' },
    { title: '社群減脂挑戰', icon: <Trophy size={24} />, desc: '與好友一起組隊參加 30 天健康挑戰，贏取專屬徽章！' },
    { title: '進階睡眠分析', icon: <Moon size={24} />, desc: '結合您的飲食習慣，由 AI 分析並改善您的睡眠品質。' },
  ];

  // 動態計算 BMI 與分類狀態
  let bmi = null;
  let bmiStatus = '';
  let bmiColor = 'text-gray-500 bg-gray-100';

  if (user?.height && user?.weight) {
    const heightM = user.height / 100;
    bmi = (user.weight / (heightM * heightM)).toFixed(1);

    if (bmi < 18.5) {
      bmiStatus = '過輕';
      bmiColor = 'text-blue-600 bg-blue-100';
    } else if (bmi < 24) {
      bmiStatus = '健康';
      bmiColor = 'text-emerald-600 bg-emerald-100';
    } else if (bmi < 27) {
      bmiStatus = '過重';
      bmiColor = 'text-orange-600 bg-orange-100';
    } else {
      bmiStatus = '肥胖';
      bmiColor = 'text-red-600 bg-red-100';
    }
  }

  return (
    <div className="space-y-6 sm:space-y-8 animate-in fade-in duration-500">
      {/* 歡迎橫幅 */}
      <div className="bg-linear-to-r from-emerald-500 via-green-500 to-teal-500 rounded-3xl p-6 sm:p-10 text-white shadow-xl relative overflow-hidden">
        <div className="relative z-10">
          <h1 className="text-2xl sm:text-4xl font-extrabold mb-2 sm:mb-3">
            您好，{user?.nickname || '健康實踐者'}！ 👋
          </h1>
          <p className="text-emerald-50 text-sm sm:text-lg max-w-xl leading-relaxed">
            今天也是邁向健康的一天。根據您的資料，建議您今天多攝取水分，並保持愉悅的心情喔！
          </p>
        </div>
        {/* 背景裝飾 */}
        <div className="absolute top-0 right-0 -mt-4 -mr-4 sm:-mt-10 sm:-mr-10 text-white opacity-10 transform rotate-12">
          <Activity className="w-48 h-48 sm:w-64 sm:h-64" />
        </div>
      </div>

      {/* 主要功能快捷鍵 */}
      <div>
        <h2 className="text-xl sm:text-2xl font-bold text-gray-800 mb-4 flex items-center">
          <Sparkles className="mr-2 text-yellow-500" /> 快速開始
        </h2>
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 sm:gap-5">
          {features.map((f) => (
            <button key={f.id} onClick={() => setCurrentView(f.id)}
              className={`p-5 sm:p-6 rounded-2xl border bg-white shadow-sm text-left transition-all duration-300 hover:-translate-y-1 hover:shadow-md ${f.hover}`}>
              <div className={`inline-block p-3 sm:p-4 rounded-xl mb-3 sm:mb-4 ${f.color}`}>
                {f.icon}
              </div>
              <h3 className="text-lg sm:text-xl font-bold text-gray-800 mb-1 sm:mb-2">{f.name}</h3>
              <p className="text-sm sm:text-base text-gray-500">{f.desc}</p>
            </button>
          ))}
        </div>
      </div>

      {/* 狀態卡片 */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-5">

        <div className="bg-white p-5 sm:p-6 rounded-2xl shadow-sm border border-gray-100 flex items-start space-x-4 sm:space-x-5">
          <div className="bg-orange-100 p-3 sm:p-4 rounded-full text-orange-600 mt-1"><User size={24} className="sm:w-7 sm:h-7" /></div>
          <div className="flex-1">
            <p className="text-xs sm:text-sm text-gray-500 font-medium mb-1">目前數值</p>
            <p className="text-lg sm:text-xl font-bold text-gray-800">
              {user?.height ? `${user.height} cm` : '--'} / {user?.weight ? `${user.weight} kg` : '--'}
            </p>

            {/* BMI 區塊與 Tooltip 懸浮提示 */}
            {bmi && (
              <div className="mt-2 relative group inline-block">
                <div className={`text-xs px-2.5 py-1.5 rounded-lg font-bold cursor-help inline-flex items-center transition-transform hover:scale-105 ${bmiColor}`}>
                  BMI: {bmi} <span className="ml-1 font-medium opacity-80">({bmiStatus})</span>
                </div>

                {/* 隱藏的懸浮提示框 (Tooltip) */}
                <div className="absolute left-0 bottom-full mb-2 hidden group-hover:block w-56 p-4 bg-slate-800 text-white text-xs rounded-xl shadow-xl z-50 pointer-events-none transform transition-all">
                  <p className="font-bold mb-2 text-emerald-400 text-sm">BMI 計算公式</p>
                  <p className="mb-3 text-slate-300 font-mono bg-slate-700 p-2 rounded-md text-center">體重(kg) ÷ 身高²(m²)</p>
                  <div className="border-t border-slate-600 pt-2 space-y-1.5">
                    <p className="flex justify-between"><span className="text-blue-300">過輕</span><span className="font-mono">{'< 18.5'}</span></p>
                    <p className="flex justify-between"><span className="text-emerald-300">健康</span><span className="font-mono">{'18.5 - 24'}</span></p>
                    <p className="flex justify-between"><span className="text-orange-300">過重</span><span className="font-mono">{'24 - 27'}</span></p>
                    <p className="flex justify-between"><span className="text-red-300">肥胖</span><span className="font-mono">{'> 27'}</span></p>
                  </div>
                  {/* 對話框的向下小箭頭 */}
                  <div className="absolute top-full left-6 border-4 border-transparent border-t-slate-800"></div>
                </div>
              </div>
            )}
          </div>
        </div>

        <div className="bg-white p-5 sm:p-6 rounded-2xl shadow-sm border border-gray-100 flex items-center space-x-4 sm:space-x-5">
          <div className="bg-slate-100 p-3 sm:p-4 rounded-full text-slate-600"><Server size={24} className="sm:w-7 sm:h-7" /></div>
          <div>
            <p className="text-xs sm:text-sm text-gray-500 font-medium mb-1">後端 API 伺服器</p>
            <div className="text-base sm:text-xl font-bold text-gray-800 flex items-center mt-1">
              {health?.status === 'ok' ? (
                <span className="text-emerald-500 flex items-center bg-emerald-50 px-2.5 py-1 rounded-full text-xs sm:text-sm">
                  <CheckCircle2 size={14} className="mr-1 sm:w-4 sm:h-4" /> 正常連線中
                </span>
              ) : (
                <span className="text-amber-500 text-xs sm:text-sm bg-amber-50 px-2.5 py-1 rounded-full">連線檢查中...</span>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* 開發藍圖 (即將推出) */}
      <div className="bg-slate-900 rounded-3xl p-6 sm:p-8 text-white mt-8 sm:mt-10 shadow-xl overflow-hidden relative">
        <div className="absolute top-0 right-0 p-4 sm:p-8 opacity-10">
          <Code2 size={150} className="sm:w-50 sm:h-50" />
        </div>
        <div className="relative z-10">
          <h2 className="text-xl sm:text-2xl font-bold mb-4 sm:mb-6 flex items-center text-emerald-400">
            <Sparkles className="mr-2" /> V2.0 開發藍圖預告
          </h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 sm:gap-6">
            {upcomingFeatures.map((feat, idx) => (
              <div key={idx} className="bg-slate-800 bg-opacity-50 p-5 rounded-2xl border border-slate-700 backdrop-blur-sm">
                <div className="text-emerald-400 mb-3">{feat.icon}</div>
                <h3 className="text-base sm:text-lg font-bold mb-1 sm:mb-2 text-slate-100">{feat.title}</h3>
                <p className="text-slate-400 text-xs sm:text-sm leading-relaxed">{feat.desc}</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

// ==========================================
// 3. 原有功能視圖 (Profile, Consult, Diet)
// ==========================================

const ProfileView = ({ user, apiFetch, showNotification, fetchProfile }) => {
  const [formData, setFormData] = useState({
    nickname: user?.nickname || '',
    height: user?.height || '',
    weight: user?.weight || '',
    dietary_restrictions: user?.dietaryRestrictions || ''
  });

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const payload = { ...formData };
      if (payload.height) payload.height = parseFloat(payload.height);
      if (payload.weight) payload.weight = parseFloat(payload.weight);

      await apiFetch('/user/profile', {
        method: 'PUT',
        body: JSON.stringify(payload)
      });
      showNotification('資料更新成功！');
      fetchProfile();
    } catch (err) {
      showNotification(err.message, 'error');
    }
  };

  return (
    <div className="max-w-3xl mx-auto bg-white p-6 sm:p-10 rounded-3xl shadow-sm border border-gray-100">
      <div className="flex items-center mb-6 sm:mb-8 border-b border-gray-100 pb-4 sm:pb-6">
        <div className="bg-emerald-100 p-3 rounded-xl text-emerald-600 mr-3 sm:mr-4">
          <User size={24} className="sm:w-7 sm:h-7" />
        </div>
        <div>
          <h2 className="text-xl sm:text-2xl font-bold text-gray-800">個人資料設定</h2>
          <p className="text-gray-500 text-xs sm:text-sm mt-1">完善您的資料以獲得更精準的 AI 建議</p>
        </div>
      </div>

      <form onSubmit={handleSubmit} className="space-y-5 sm:space-y-6">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-5 sm:gap-6">
          <div>
            <label className="block text-sm font-semibold text-gray-700 mb-1 sm:mb-2">暱稱</label>
            <input type="text" value={formData.nickname} onChange={e => setFormData({ ...formData, nickname: e.target.value })}
              className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-emerald-500 outline-none bg-gray-50 focus:bg-white transition-colors" />
          </div>
          <div>
            <label className="block text-sm font-semibold text-gray-700 mb-1 sm:mb-2">電子郵件 (唯讀)</label>
            <input type="email" value={user?.email || ''} readOnly disabled
              className="w-full px-4 py-3 border border-gray-200 bg-gray-100 rounded-xl text-gray-500 cursor-not-allowed" />
          </div>
          <div>
            <label className="block text-sm font-semibold text-gray-700 mb-1 sm:mb-2">身高 (cm)</label>
            <input type="number" step="0.1" value={formData.height} onChange={e => setFormData({ ...formData, height: e.target.value })}
              className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-emerald-500 outline-none bg-gray-50 focus:bg-white transition-colors" />
          </div>
          <div>
            <label className="block text-sm font-semibold text-gray-700 mb-1 sm:mb-2">體重 (kg)</label>
            <input type="number" step="0.1" value={formData.weight} onChange={e => setFormData({ ...formData, weight: e.target.value })}
              className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-emerald-500 outline-none bg-gray-50 focus:bg-white transition-colors" />
          </div>
        </div>
        <div>
          <label className="block text-sm font-semibold text-gray-700 mb-1 sm:mb-2">飲食禁忌 / 過敏源 / 喜歡的食物</label>
          <textarea value={formData.dietary_restrictions} onChange={e => setFormData({ ...formData, dietary_restrictions: e.target.value })} rows="4"
            className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-emerald-500 outline-none resize-none bg-gray-50 focus:bg-white transition-colors"
            placeholder="例如：不吃花生, 素食，或對海鮮過敏..."></textarea>
        </div>
        <div className="flex justify-end pt-4 sm:pt-6 border-t border-gray-100">
          <button type="submit" className="w-full sm:w-auto bg-emerald-600 text-white px-8 py-3 rounded-xl font-bold hover:bg-emerald-700 transition-colors shadow-md">
            儲存變更
          </button>
        </div>
      </form>
    </div>
  );
};

const ConsultView = ({ user, apiFetch, showNotification, fetchProfile }) => {
  const [question, setQuestion] = useState('');
  const [chatHistory, setChatHistory] = useState([]);
  const [isThinking, setIsThinking] = useState(false);
  const endOfChatRef = useRef(null);

  // 接收後端陣列 (支援 JS 的 camelCase 或是 Rust 的 snake_case)
  const historyData = user?.aiConsultations || user?.ai_consultations;

  // 1. 同步與自動排序歷史紀錄
  useEffect(() => {
    if (historyData) {
      const sortedHistory = [...historyData].sort((a, b) => {
        const timeA = new Date(a.createdAt || a.created_at || 0).getTime();
        const timeB = new Date(b.createdAt || b.created_at || 0).getTime();
        return timeA - timeB;
      });
      setChatHistory(sortedHistory);
    }
  }, [historyData]);

  useEffect(() => {
    endOfChatRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [chatHistory, isThinking]);

  const handleAsk = async (e) => {
    e.preventDefault();
    if (!question.trim()) return;

    const newQ = question;
    setQuestion('');
    setIsThinking(true);

    const tempHistory = [...chatHistory, { id: Date.now(), question: newQ, aiResponse: null }];
    setChatHistory(tempHistory);

    try {
      const data = await apiFetch('/consult', {
        method: 'POST',
        body: JSON.stringify({ question: newQ })
      });

      setChatHistory(prev => {
        const updated = [...prev];
        updated[updated.length - 1].aiResponse = data.reply || data.aiResponse || data.ai_response || data.answer || data.response || "無法解析後端回應，請檢查 API 欄位名稱";
        return updated;
      });
      fetchProfile();
    } catch (err) {
      showNotification(err.message, 'error');
      setChatHistory(prev => prev.slice(0, -1));
    } finally {
      setIsThinking(false);
    }
  };

  return (
    <div className="max-w-4xl -mx- bg-white sm:rounded-3xl shadow-sm border-x sm:border border-gray-100 overflow-hidden flex flex-col h-[calc(100vh-120px)] sm:h-[calc(100vh-140px)] relative -mx-4 sm:mx-0">
      <div className="bg-linear-to-r from-emerald-600 to-teal-600 p-4 sm:p-5 text-white flex items-center shadow-md relative z-10">
        <MessageSquare className="mr-3" size={24} />
        <div>
          <h2 className="text-lg sm:text-xl font-bold">專屬 AI 營養師</h2>
          <p className="text-emerald-100 text-xs mt-0.5 sm:mt-1">24 小時在線，結合您的身體數據提供建議</p>
        </div>
      </div>

      <div className="flex-1 p-4 sm:p-6 overflow-y-auto bg-slate-50 space-y-4 sm:space-y-6">
        {chatHistory.length === 0 && !isThinking ? (
          <div className="flex flex-col items-center justify-center h-full text-gray-400">
            <div className="bg-white p-5 sm:p-6 rounded-full shadow-sm mb-4">
              <MessageSquare size={40} className="text-emerald-300 sm:w-12 sm:h-12" />
            </div>
            <p className="font-medium text-gray-500 text-sm sm:text-base">開始向 AI 營養師提問吧！</p>
            <div className="mt-4 sm:mt-6 space-y-2">
              <p className="text-xs sm:text-sm bg-white px-3 sm:px-4 py-2 rounded-full border border-gray-100 shadow-sm cursor-pointer hover:bg-emerald-50 transition" onClick={() => setQuestion('我剛重訓完，宵夜推薦吃什麼？')}>💡 「我剛重訓完，宵夜推薦吃什麼？」</p>
              <p className="text-xs sm:text-sm bg-white px-3 sm:px-4 py-2 rounded-full border border-gray-100 shadow-sm cursor-pointer hover:bg-emerald-50 transition" onClick={() => setQuestion('請問 168 斷食期間可以喝無糖豆漿嗎？')}>💡 「請問 168 斷食期間可以喝無糖豆漿嗎？」</p>
            </div>
          </div>
        ) : (
          chatHistory.map((chat, idx) => {
            const aiText = chat.aiResponse || chat.ai_response || chat.response || chat.answer || chat.reply;
            const userText = chat.question || chat.query || chat.message;

            return (
              <div key={chat.id || idx} className="space-y-3 sm:space-y-4">
                <div className="flex justify-end">
                  <div className="bg-emerald-500 text-white px-4 py-2.5 sm:px-5 sm:py-3 rounded-2xl rounded-tr-sm max-w-[85%] sm:max-w-[80%] shadow-sm leading-relaxed text-sm sm:text-base">
                    {userText}
                  </div>
                </div>
                {aiText && (
                  <div className="flex justify-start items-end space-x-2">
                    <div className="bg-linear-to-br from-teal-500 to-emerald-500 p-1.5 sm:p-2 rounded-full text-white shadow-sm flex-shrink-0 mb-1">
                      <Activity size={14} className="sm:w-4 sm:h-4" />
                    </div>
                    <div className="bg-white border border-gray-100 text-gray-700 px-4 py-3 sm:px-5 sm:py-4 rounded-2xl rounded-tl-sm max-w-[85%] sm:max-w-[80%] shadow-sm leading-relaxed text-sm sm:text-base overflow-x-auto">
                      <ReactMarkdown
                        components={{
                          p: ({ _, ...props }) => <p className="mb-2 last:mb-0 leading-relaxed" {...props} />,
                          ul: ({ _, ...props }) => <ul className="list-disc ml-5 mb-3 space-y-1" {...props} />,
                          ol: ({ node, ...props }) => <ol className="list-decimal ml-5 mb-3 space-y-1" {...props} />,
                          li: ({ node, ...props }) => <li className="" {...props} />,
                          h1: ({ node, ...props }) => <h1 className="text-xl font-bold mb-3 mt-4 text-emerald-800 border-b pb-1" {...props} />,
                          h2: ({ node, ...props }) => <h2 className="text-lg font-bold mb-2 mt-3 text-emerald-800" {...props} />,
                          h3: ({ node, ...props }) => <h3 className="text-base font-bold mb-2 mt-2 text-emerald-700" {...props} />,
                          strong: ({ node, ...props }) => <strong className="font-bold text-emerald-700" {...props} />,
                          em: ({ node, ...props }) => <em className="italic text-gray-600" {...props} />,
                          code: ({ node, inline, ...props }) =>
                            inline
                              ? <code className="bg-gray-100 text-emerald-600 px-1 py-0.5 rounded text-sm font-mono" {...props} />
                              : <pre className="bg-slate-800 text-slate-50 p-3 rounded-lg overflow-x-auto my-3 text-sm font-mono"><code {...props} /></pre>,
                          blockquote: ({ node, ...props }) => <blockquote className="border-l-4 border-emerald-300 pl-3 italic text-gray-500 my-3 bg-emerald-50 py-1 rounded-r-lg" {...props} />,
                          a: ({ node, ...props }) => <a className="text-emerald-600 underline hover:text-emerald-800" target="_blank" rel="noopener noreferrer" {...props} />
                        }}
                      >
                        {aiText}
                      </ReactMarkdown>
                    </div>
                  </div>
                )}
              </div>
            );
          })
        )}
        {isThinking && (
          <div className="flex justify-start items-end space-x-2">
            <div className="bg-linear-to-b from-teal-500 to-emerald-500 p-1.5 sm:p-2 rounded-full text-white shadow-sm flex-shrink-0 mb-1">
              <Activity size={14} className="sm:w-4 sm:h-4" />
            </div>
            <div className="bg-white border border-gray-100 p-3 sm:p-4 rounded-2xl rounded-tl-sm shadow-sm flex items-center space-x-1.5 sm:space-x-2">
              <div className="w-2 h-2 sm:w-2.5 sm:h-2.5 bg-emerald-400 rounded-full animate-bounce" style={{ animationDelay: '0ms' }}></div>
              <div className="w-2 h-2 sm:w-2.5 sm:h-2.5 bg-emerald-400 rounded-full animate-bounce" style={{ animationDelay: '150ms' }}></div>
              <div className="w-2 h-2 sm:w-2.5 sm:h-2.5 bg-emerald-400 rounded-full animate-bounce" style={{ animationDelay: '300ms' }}></div>
            </div>
          </div>
        )}
        <div ref={endOfChatRef} />
      </div>

      <div className="p-3 sm:p-4 bg-white border-t border-gray-100 shadow-[0_-4px_6px_-1px_rgba(0,0,0,0.05)] relative z-10">
        <form onSubmit={handleAsk} className="flex space-x-2 sm:space-x-3 max-w-4xl mx-auto">
          <input type="text" value={question} onChange={e => setQuestion(e.target.value)}
            placeholder="輸入您的健康或飲食問題..." disabled={isThinking}
            className="flex-1 px-4 py-3 sm:px-5 sm:py-4 border border-gray-200 rounded-2xl focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 outline-none bg-slate-50 focus:bg-white transition-colors text-sm sm:text-base" />
          <button type="submit" disabled={isThinking || !question.trim()}
            className="bg-emerald-600 text-white px-5 sm:px-6 py-3 sm:py-4 rounded-2xl hover:bg-emerald-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed shadow-md flex items-center justify-center">
            <Send size={20} className={`sm:w-6 sm:h-6 ${question.trim() && !isThinking ? 'animate-pulse' : ''}`} />
          </button>
        </form>
      </div>
    </div>
  );
};

// 🔽 這是被更新的 DietView (飲食辨識視圖)
const DietView = ({ apiFetch, showNotification }) => {
  const [selectedFile, setSelectedFile] = useState(null);
  const [previewUrl, setPreviewUrl] = useState(null);
  const [result, setResult] = useState(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);

  // 相機相關 State 與 Ref
  const [isCameraOpen, setIsCameraOpen] = useState(false);
  const videoRef = useRef(null);
  const streamRef = useRef(null);

  useEffect(() => {
    return () => stopCamera();
  }, []);

  const startCamera = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: 'environment' }
      });
      streamRef.current = stream;
      setIsCameraOpen(true);
      if (videoRef.current) {
        videoRef.current.srcObject = stream;
      }
    } catch (err) {
      console.error("相機存取錯誤:", err);
      showNotification('無法存取相機，請確認已授予瀏覽器相機權限。', 'error');
    }
  };

  const stopCamera = () => {
    if (streamRef.current) {
      streamRef.current.getTracks().forEach(track => track.stop());
      streamRef.current = null;
    }
    setIsCameraOpen(false);
  };

  const capturePhoto = () => {
    if (videoRef.current) {
      const canvas = document.createElement('canvas');
      canvas.width = videoRef.current.videoWidth;
      canvas.height = videoRef.current.videoHeight;
      const ctx = canvas.getContext('2d');
      ctx.drawImage(videoRef.current, 0, 0, canvas.width, canvas.height);

      canvas.toBlob((blob) => {
        const file = new File([blob], "camera_capture.jpg", { type: "image/jpeg" });
        setSelectedFile(file);
        setPreviewUrl(URL.createObjectURL(file));
        setResult(null);
        stopCamera();
      }, 'image/jpeg');
    }
  };

  const handleFileChange = (e) => {
    const file = e.target.files[0];
    if (file) {
      setSelectedFile(file);
      setPreviewUrl(URL.createObjectURL(file));
      setResult(null);
      stopCamera();
    }
  };

  const clearSelection = () => {
    setSelectedFile(null);
    setPreviewUrl(null);
    setResult(null);
  };

  const handleUpload = async () => {
    if (!selectedFile) return;
    setIsAnalyzing(true);
    const formData = new FormData();
    formData.append('image', selectedFile);

    try {
      // 呼叫更新後的 /diet API
      const data = await apiFetch('/diet', {
        method: 'POST',
        body: formData
      });

      // 🔽 直接將回傳的 JSON (CalorieResponse) 存入 Result
      setResult(data);
      showNotification('AI 視覺分析完成！');
    } catch (err) {
      showNotification(err.message, 'error');
    } finally {
      setIsAnalyzing(false);
    }
  };

  return (
    <div className="max-w-3xl mx-auto space-y-6">
      <div className="bg-white p-6 sm:p-10 rounded-3xl shadow-sm border border-gray-100">
        <div className="flex items-center mb-6 sm:mb-8 border-b border-gray-100 pb-4 sm:pb-6">
          <div className="bg-blue-100 p-3 rounded-xl text-blue-600 mr-3 sm:mr-4">
            <Camera size={24} className="sm:w-7 sm:h-7" />
          </div>
          <div>
            <h2 className="text-xl sm:text-2xl font-bold text-gray-800">飲食 YOLO 視覺辨識</h2>
            <p className="text-gray-500 text-xs sm:text-sm mt-1">上傳或拍攝食物照片，讓模型自動辨識成份與熱量</p>
          </div>
        </div>

        {/* 主要上傳 / 相機區塊 */}
        <div className="mb-6">
          {isCameraOpen ? (
            <div className="relative bg-black rounded-3xl overflow-hidden shadow-inner aspect-video flex flex-col items-center justify-center">
              <video
                ref={videoRef}
                autoPlay
                playsInline
                className="w-full h-full object-cover"
              />
              <button
                onClick={stopCamera}
                className="absolute top-4 right-4 bg-black/50 text-white p-2 rounded-full hover:bg-black/70 transition"
              >
                <X size={20} />
              </button>
              <div className="absolute bottom-6 left-0 right-0 flex justify-center">
                <button
                  onClick={capturePhoto}
                  className="bg-white text-blue-600 p-4 rounded-full shadow-lg hover:scale-110 transition-transform flex items-center justify-center border-4 border-blue-200"
                >
                  <Camera size={32} />
                </button>
              </div>
            </div>
          ) : previewUrl ? (
            <div className="relative border-2 border-gray-200 rounded-3xl p-2 sm:p-4 text-center bg-slate-50">
              <img src={previewUrl} alt="Preview" className="mx-auto max-h-[50vh] sm:max-h-80 rounded-2xl object-contain shadow-sm" />
              <button
                onClick={clearSelection}
                className="absolute top-4 right-4 sm:top-6 sm:right-6 bg-white text-gray-600 p-2 rounded-full shadow-md hover:bg-red-50 hover:text-red-500 transition"
                title="移除照片"
              >
                <X size={20} />
              </button>
            </div>
          ) : (
            <div className="border-2 border-dashed border-gray-300 rounded-3xl p-8 sm:p-12 text-center bg-slate-50 flex flex-col items-center justify-center">
              <div className="bg-blue-50 p-6 rounded-full mb-6">
                <UploadCloud size={48} className="text-blue-500 sm:w-14 sm:h-14" />
              </div>
              <p className="font-bold text-lg sm:text-xl text-gray-700 mb-6">請選擇照片來源</p>

              <div className="flex flex-col sm:flex-row gap-4 w-full justify-center max-w-sm">
                <button
                  onClick={() => document.getElementById('fileUpload').click()}
                  className="flex-1 bg-white border border-gray-200 text-gray-700 py-3 px-4 rounded-xl font-bold shadow-sm hover:bg-blue-50 hover:border-blue-200 hover:text-blue-600 transition flex items-center justify-center"
                >
                  <ImageIcon size={20} className="mr-2" /> 從相簿選擇
                </button>
                <input type="file" id="fileUpload" className="hidden" accept="image/*" onChange={handleFileChange} />

                <button
                  onClick={startCamera}
                  className="flex-1 bg-white border border-gray-200 text-gray-700 py-3 px-4 rounded-xl font-bold shadow-sm hover:bg-emerald-50 hover:border-emerald-200 hover:text-emerald-600 transition flex items-center justify-center"
                >
                  <Camera size={20} className="mr-2" /> 開啟相機
                </button>
              </div>
              <p className="text-xs text-gray-400 mt-6">支援 JPG, PNG, WEBP (最大 5MB)</p>
            </div>
          )}
        </div>

        {/* 送出分析按鈕 */}
        <div className="flex justify-center mt-6">
          <button onClick={handleUpload} disabled={!selectedFile || isAnalyzing || isCameraOpen}
            className="w-full sm:w-auto bg-blue-600 text-white px-8 sm:px-12 py-3 sm:py-4 rounded-2xl hover:bg-blue-700 transition-all font-bold text-base sm:text-lg disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center shadow-lg hover:shadow-xl">
            {isAnalyzing ? (
              <><Activity className="animate-spin mr-2 sm:mr-3" size={24} /> 模型推論中...</>
            ) : (
              <><Sparkles className="mr-2 sm:mr-3" size={24} /> 開始 AI 辨識</>
            )}
          </button>
        </div>
      </div>

      {/* 🔽 辨識結果 (配合新的 JSON 結構) */}
      {result && (
        <div className="bg-blue-50 p-6 sm:p-8 rounded-3xl border border-blue-100 animate-in slide-in-from-bottom-4 shadow-sm">

          <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-4 sm:mb-6 gap-4">
            <h3 className="text-lg sm:text-xl font-bold text-blue-900 flex items-center">
              <CheckCircle2 className="mr-2 text-blue-600" size={24} /> {result.message}
            </h3>
            {/* 新增：顯示總熱量 */}
            {result.total_calories !== undefined && (
              <div className="bg-orange-100 text-orange-700 px-4 py-2 rounded-xl font-bold text-lg flex items-center shadow-sm w-full sm:w-auto justify-center">
                總熱量: <span className="text-2xl ml-2 mr-1">{result.total_calories}</span> ±50 kcal
              </div>
            )}
          </div>

          <div className="space-y-3 sm:space-y-4">
            {result.detected_items && result.detected_items.length > 0 ? result.detected_items.map((det, idx) => (
              <div key={idx} className="bg-white p-4 sm:p-5 rounded-2xl shadow-sm border border-blue-100 flex flex-col sm:flex-row sm:justify-between sm:items-center transform hover:scale-[1.01] transition-transform gap-3 sm:gap-0">
                <div className="flex items-center">
                  <div className="bg-blue-100 p-2 rounded-lg mr-3 sm:mr-4 text-blue-700">
                    <Database size={18} className="sm:w-5 sm:h-5" />
                  </div>
                  <div>
                    <span className="font-bold text-gray-800 capitalize text-base sm:text-lg block">{det.class}</span>
                    <span className="text-xs text-gray-500">信心度 {(det.confidence * 100).toFixed(0)}%</span>
                  </div>
                </div>

                {/* 新增：顯示預估重量與熱量 */}
                <div className="flex items-center space-x-3 sm:space-x-4 bg-slate-50 sm:bg-transparent p-3 sm:p-0 rounded-xl">
                  <div className="text-sm font-medium text-gray-600">
                    <span className="text-gray-400 text-xs block sm:inline sm:mr-1">預估重量</span>
                    {det.estimated_weight_g} g
                  </div>
                  <div className="h-8 w-px bg-gray-200 hidden sm:block"></div>
                  <div className="text-sm font-bold text-orange-600">
                    <span className="text-orange-300 text-xs block sm:inline sm:mr-1">熱量</span>
                    {det.calories}±20 kcal
                  </div>
                </div>
              </div>
            )) : (
              <div className="bg-white p-6 sm:p-8 rounded-2xl text-center border border-blue-100">
                <Activity size={40} className="sm:w-12 sm:h-12 mx-auto text-gray-300 mb-3" />
                <p className="text-gray-600 font-medium text-sm sm:text-base">未能辨識出任何明確的食物模型，請換一張更清晰的照片試試！</p>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

// ==========================================
// 4. 新增：關於團隊視圖
// ==========================================

const AboutView = () => {
  const team = [
    { name: 'Dr. Sarah Lin', role: '首席營養顧問', desc: '擁有 10 年臨床營養學經驗，負責調教 AI 模型的營養學知識庫。', icon: <Sparkles className="text-yellow-500" /> },
    { name: 'Alex Chen', role: 'AI 演算法工程師', desc: '專注於 YOLO 視覺模型優化與 LLM (大型語言模型) 的 Prompt Engineering。', icon: <Code2 className="text-blue-500" /> },
    { name: 'Jamie Wang', role: '全端開發者', desc: '負責架構 React 前端與 Flask 後端 API，致力於提供最流暢的使用者體驗。', icon: <Server className="text-emerald-500" /> }
  ];

  return (
    <div className="max-w-5xl mx-auto space-y-8 sm:space-y-10 animate-in fade-in duration-500">
      {/* 願景區塊 */}
      <div className="bg-slate-900 rounded-3xl p-8 sm:p-10 text-center text-white shadow-2xl relative overflow-hidden">
        <div className="absolute inset-0 opacity-20 bg-[radial-gradient(circle_at_center,_var(--tw-gradient-stops))] from-emerald-500 via-slate-900 to-slate-900"></div>
        <div className="relative z-10">
          <div className="inline-block bg-slate-800 p-3 sm:p-4 rounded-2xl mb-4 sm:mb-6 shadow-inner">
            <Users size={36} className="sm:w-12 sm:h-12 text-emerald-400" />
          </div>
          <h1 className="text-2xl sm:text-4xl font-extrabold mb-3 sm:mb-4 text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-teal-300">
            科技賦能，讓健康觸手可及
          </h1>
          <p className="text-slate-300 max-w-2xl mx-auto text-sm sm:text-lg leading-relaxed">
            我們的使命是將頂尖的 AI 視覺辨識與自然語言處理技術，轉化為每個人口袋裡的專屬健康顧問。
            不必再苦苦計算卡路里，拍張照、問個問題，健康生活就是這麼簡單。
          </p>
        </div>
      </div>

      {/* 團隊成員 */}
      <div>
        <h2 className="text-xl sm:text-2xl font-bold text-gray-800 mb-4 sm:mb-6 text-center">認識我們的核心團隊</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 sm:gap-6">
          {team.map((member, idx) => (
            <div key={idx} className="bg-white p-6 sm:p-8 rounded-3xl shadow-sm border border-gray-100 hover:shadow-lg transition-shadow text-center">
              <div className="w-20 h-20 sm:w-24 sm:h-24 bg-slate-50 rounded-full mx-auto mb-4 sm:mb-6 flex items-center justify-center shadow-inner border-2 border-slate-100">
                {member.icon}
              </div>
              <h3 className="text-lg sm:text-xl font-bold text-gray-800 mb-1">{member.name}</h3>
              <p className="text-emerald-600 font-medium text-xs sm:text-sm mb-3 sm:mb-4">{member.role}</p>
              <p className="text-gray-500 text-xs sm:text-sm leading-relaxed">{member.desc}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

// ==========================================
// 5. 新增：API 文件視圖
// ==========================================

const ApiDocsView = () => {
  const endpoints = [
    { method: 'POST', path: '/auth/login', title: '使用者登入', desc: '驗證使用者身分並返回 JWT Token 及使用者資訊。', tag: 'Auth' },
    { method: 'POST', path: '/auth/register', title: '使用者註冊', desc: '使用電子郵件及密碼建立新帳戶。', tag: 'Auth' },
    { method: 'GET', path: '/user/profile', title: '取得個人資料', desc: '取得基本資料、身體數值及 AI 諮詢紀錄。需 Bearer Token。', tag: 'Users' },
    { method: 'PUT', path: '/user/profile', title: '更新個人資料', desc: '更新身高、體重或飲食禁忌。', tag: 'Users' },
    { method: 'POST', path: '/consult', title: 'AI 營養諮詢', desc: '發送問題給 AI，自動附加個人檔案產生專屬建議。', tag: 'Nutrition' },
    { method: 'POST', path: '/diet', title: '相機辨識 (YOLO)', desc: '上傳圖片 (multipart/form-data) 進行物件偵測與熱量解析。', tag: 'Nutrition' },
    { method: 'GET', path: '/', title: '系統狀態檢查', desc: '返回 API 服務目前的運作與連線狀態。', tag: 'System' },
  ];

  const getMethodStyle = (method) => {
    switch (method) {
      case 'GET': return 'bg-blue-100 text-blue-700 border-blue-200';
      case 'POST': return 'bg-emerald-100 text-emerald-700 border-emerald-200';
      case 'PUT': return 'bg-amber-100 text-amber-700 border-amber-200';
      default: return 'bg-gray-100 text-gray-700';
    }
  };

  return (
    <div className="max-w-4xl mx-auto space-y-6 sm:space-y-8 animate-in slide-in-from-bottom-4 duration-500">
      <div className="bg-white p-6 sm:p-10 rounded-3xl shadow-sm border border-gray-100">
        <div className="flex flex-col sm:flex-row sm:items-center mb-6">
          <div className="bg-slate-900 p-3 sm:p-4 rounded-2xl text-emerald-400 sm:mr-5 shadow-lg w-fit mb-4 sm:mb-0">
            <BookOpen size={28} className="sm:w-8 sm:h-8" />
          </div>
          <div>
            <h1 className="text-2xl sm:text-3xl font-extrabold text-gray-800">RESTful API 規格</h1>
            <p className="text-gray-500 mt-1 sm:mt-2 text-sm sm:text-base">OpenAPI 3.0 實作 - 提供完整的使用者認證、飲食記錄與 AI 服務整合。</p>
          </div>
        </div>

        <div className="bg-slate-50 p-4 sm:p-6 rounded-2xl border border-slate-200 mb-6 sm:mb-8 font-mono text-xs sm:text-sm text-slate-700 flex flex-col sm:flex-row sm:space-x-8">
          <div><span className="font-bold text-slate-900">Base URL:</span> {API_BASE}</div>
          <div className="mt-2 sm:mt-0"><span className="font-bold text-slate-900">Auth:</span> Bearer JWT</div>
        </div>

        <div className="space-y-4">
          {endpoints.map((ep, idx) => (
            <div key={idx} className="border border-gray-200 rounded-2xl overflow-hidden hover:shadow-md transition-shadow">
              <div className="bg-gray-50 p-4 flex flex-col sm:flex-row sm:items-center justify-between border-b border-gray-100">
                <div className="flex items-center space-x-3 sm:space-x-4 mb-3 sm:mb-0">
                  <span className={`px-3 sm:px-4 py-1 sm:py-1.5 rounded-lg text-xs sm:text-sm font-bold border ${getMethodStyle(ep.method)}`}>
                    {ep.method}
                  </span>
                  <span className="font-mono font-semibold text-gray-700 break-all text-xs sm:text-sm">{ep.path}</span>
                </div>
                <span className="text-[10px] sm:text-xs font-bold text-gray-400 uppercase tracking-wider bg-white px-2 sm:px-3 py-1 rounded-full border border-gray-100 self-start sm:self-auto">
                  {ep.tag}
                </span>
              </div>
              <div className="p-4 bg-white">
                <p className="font-bold text-gray-800 mb-1 text-sm sm:text-base">{ep.title}</p>
                <p className="text-xs sm:text-sm text-gray-600 leading-relaxed">{ep.desc}</p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};


// ==========================================
// 主應用程式元件 (App)
// ==========================================

export default function App() {
  const [token, setToken] = useState(localStorage.getItem('token') || null);
  const [user, setUser] = useState(null);
  const [currentView, setCurrentView] = useState('login');
  const [notification, setNotification] = useState(null);

  useEffect(() => {
    if (token && (currentView === 'login' || currentView === 'register')) {
      fetchProfile();
      setCurrentView('dashboard');
    }
  }, [token]);

  const showNotification = (msg, type = 'success') => {
    setNotification({ msg, type });
    setTimeout(() => setNotification(null), 3000);
  };

  const handleLogout = () => {
    setToken(null);
    setUser(null);
    localStorage.removeItem('token');
    setCurrentView('login');
  };

  const apiFetch = async (endpoint, options = {}) => {
    const headers = { ...options.headers };
    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }
    if (!(options.body instanceof FormData)) {
      headers['Content-Type'] = 'application/json';
    } else {
      delete headers['Content-Type'];
    }

    const response = await fetch(`${API_BASE}${endpoint}`, { ...options, headers });
    if (response.status === 401) {
      handleLogout();
      throw new Error('登入已過期，請重新登入');
    }
    const data = await response.json();
    if (!response.ok) throw new Error(data.error || '請求失敗');
    return data;
  };

  const fetchProfile = async () => {
    try {
      const data = await apiFetch('/user/profile');
      setUser(data);
    } catch (err) {
      console.error(err);
    }
  };

  // 登入或註冊畫面
  if (currentView === 'login' || currentView === 'register') {
    return (
      <div className="min-h-screen flex items-center justify-center bg-[radial-gradient(ellipse_at_top_right,_var(--tw-gradient-stops))] from-emerald-50 via-gray-50 to-blue-50 relative p-4 overflow-hidden">
        {/* 背景裝飾 */}
        <div className="absolute -top-10 -left-10 sm:top-10 sm:left-10 text-emerald-100 opacity-40"><Activity size={250} /></div>
        <div className="absolute -bottom-10 -right-10 sm:bottom-10 sm:right-10 text-blue-100 opacity-40"><User size={250} /></div>

        {notification && (
          <div className={`absolute top-6 left-1/2 transform -translate-x-1/2 px-6 sm:px-8 py-3 sm:py-4 rounded-2xl text-white shadow-xl transition-all z-50 font-bold flex items-center w-[90%] sm:w-auto text-sm sm:text-base ${notification.type === 'error' ? 'bg-red-500' : 'bg-slate-800'}`}>
            {notification.type === 'error' ? <Activity className="mr-2 flex-shrink-0" /> : <CheckCircle2 className="mr-2 flex-shrink-0" />}
            <span className="truncate">{notification.msg}</span>
          </div>
        )}

        {currentView === 'login' ? (
          <LoginView apiFetch={apiFetch} setToken={setToken} setCurrentView={setCurrentView} showNotification={showNotification} />
        ) : (
          <RegisterView apiFetch={apiFetch} setToken={setToken} setCurrentView={setCurrentView} showNotification={showNotification} />
        )}
      </div>
    );
  }

  // 電腦版/平板上方 Navbar 按鈕
  const DesktopNavButton = ({ id, icon: Icon, label }) => (
    <button onClick={() => setCurrentView(id)}
      className={`p-2 sm:p-2.5 rounded-xl flex items-center transition-all font-semibold text-sm sm:text-base ${currentView === 'dashboard' && id === 'dashboard' || currentView === id ? 'bg-emerald-50 text-emerald-600 shadow-sm' : 'text-gray-500 hover:bg-gray-100 hover:text-gray-800'}`}>
      <Icon size={18} className="sm:w-5 sm:h-5 sm:mr-2" /> <span className="hidden lg:inline">{label}</span>
    </button>
  );

  // 手機版底部 Navbar 按鈕
  const MobileNavButton = ({ id, icon: Icon, label }) => {
    const isActive = currentView === id || (currentView === 'dashboard' && id === 'dashboard');
    return (
      <button onClick={() => setCurrentView(id)}
        className={`flex flex-col items-center justify-center w-full py-1 ${isActive ? 'text-emerald-600' : 'text-gray-400'}`}>
        <div className={`p-1.5 rounded-xl mb-1 transition-all ${isActive ? 'bg-emerald-50' : ''}`}>
          <Icon size={22} className={isActive ? 'stroke-[2.5px]' : ''} />
        </div>
        <span className={`text-[10px] font-bold ${isActive ? 'text-emerald-600' : 'text-gray-500'}`}>{label}</span>
      </button>
    );
  };

  return (
    <div className="min-h-screen bg-slate-50 flex flex-col font-sans">
      {/* 頂部導覽列 (電腦版/平板版) */}
      <nav className="bg-white border-b border-gray-200 sticky top-0 z-40 shadow-[0_2px_10px_rgba(0,0,0,0.02)] hidden sm:block">
        <div className="max-w-7xl mx-auto px-4 sm:px-6">
          <div className="flex justify-between h-16 sm:h-20">
            {/* Logo */}
            <div className="flex items-center space-x-3 cursor-pointer group" onClick={() => setCurrentView('dashboard')}>
              <div className="bg-gradient-to-br from-emerald-400 to-green-600 p-2 sm:p-2.5 rounded-xl text-white shadow-md group-hover:shadow-lg transition-all">
                <Activity size={24} />
              </div>
              <span className="font-extrabold text-xl sm:text-2xl text-transparent bg-clip-text bg-gradient-to-r from-slate-800 to-slate-600 tracking-tight">
                健康飲食APP 網頁版
              </span>
            </div>

            {/* 選單列表 */}
            <div className="flex items-center space-x-1 sm:space-x-2">
              <DesktopNavButton id="dashboard" icon={Home} label="首頁" />
              <DesktopNavButton id="consult" icon={MessageSquare} label="AI諮詢" />
              <DesktopNavButton id="diet" icon={Camera} label="飲食辨識" />
              <DesktopNavButton id="profile" icon={User} label="設定" />
              <div className="h-6 w-px bg-gray-200 mx-1 sm:mx-2"></div>
              <DesktopNavButton id="about" icon={Users} label="團隊" />
              <DesktopNavButton id="api-docs" icon={BookOpen} label="API" />

              <div className="h-6 w-px bg-gray-200 mx-1 sm:mx-2"></div>
              <button onClick={handleLogout} className="p-2 sm:p-2.5 text-red-500 hover:bg-red-50 rounded-xl flex items-center transition-colors font-semibold">
                <LogOut size={18} className="sm:w-5 sm:h-5" /> <span className="hidden lg:inline ml-2 text-sm sm:text-base">登出</span>
              </button>
            </div>
          </div>
        </div>
      </nav>

      {/* 手機版頂部 Header (只顯示 Logo 和登出) */}
      <header className="sm:hidden bg-white border-b border-gray-200 sticky top-0 z-40 px-4 h-14 flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <div className="bg-gradient-to-br from-emerald-400 to-green-600 p-1.5 rounded-lg text-white">
            <Activity size={18} />
          </div>
          <span className="font-extrabold text-lg text-slate-800 tracking-tight">
            健康管家
          </span>
        </div>
        <button onClick={handleLogout} className="p-2 text-red-500 hover:bg-red-50 rounded-lg">
          <LogOut size={20} />
        </button>
      </header>

      {/* 主要內容區 */}
      <main className="flex-1 max-w-7xl w-full mx-auto p-4 sm:p-6 md:p-8 lg:p-10 pb-24 sm:pb-10 relative">
        {notification && (
          <div className={`fixed top-16 sm:top-24 left-1/2 transform -translate-x-1/2 z-50 px-6 sm:px-8 py-3 sm:py-4 rounded-full text-white shadow-2xl transition-all font-bold flex items-center w-max max-w-[90vw] text-sm sm:text-base ${notification.type === 'error' ? 'bg-red-500' : 'bg-slate-800'}`}>
            {notification.type === 'error' ? <Activity className="mr-2 flex-shrink-0" /> : <CheckCircle2 className="mr-2 flex-shrink-0" />}
            <span className="truncate">{notification.msg}</span>
          </div>
        )}

        {/* 路由控制 */}
        {currentView === 'dashboard' && <DashboardView user={user} setCurrentView={setCurrentView} />}
        {currentView === 'profile' && <ProfileView user={user} apiFetch={apiFetch} showNotification={showNotification} fetchProfile={fetchProfile} />}
        {currentView === 'consult' && <ConsultView user={user} apiFetch={apiFetch} showNotification={showNotification} fetchProfile={fetchProfile} />}
        {currentView === 'diet' && <DietView apiFetch={apiFetch} showNotification={showNotification} />}
        {currentView === 'about' && <AboutView />}
        {currentView === 'api-docs' && <ApiDocsView />}
      </main>

      {/* 手機版專屬底部導覽列 (Bottom Navigation) */}
      <nav className="sm:hidden fixed bottom-0 left-0 right-0 bg-white border-t border-gray-200 z-50 pb-safe">
        <div className="flex justify-around items-center h-16 px-2">
          <MobileNavButton id="dashboard" icon={Home} label="首頁" />
          <MobileNavButton id="consult" icon={MessageSquare} label="AI諮詢" />
          <MobileNavButton id="diet" icon={Camera} label="相機辨識" />
          <MobileNavButton id="about" icon={Users} label="團隊" />
          <MobileNavButton id="profile" icon={User} label="設定" />
        </div>
      </nav>
    </div>
  );
}
