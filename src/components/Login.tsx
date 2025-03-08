import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { useContext } from "react";
import { AuthContext } from "../App";
import Moodlelogo from "../assets/icons/Moodlelogo.svg"
export default function Login () {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const { loggedIn, setLoggedIn} = useContext(AuthContext);

  const handleSubmit = async() => {
    try {
      let response: string = await invoke('login', { payload: JSON.stringify({ username: username, password: password }) });
      console.log(response);
      localStorage.setItem("username", response);
      setLoggedIn(true);
    } catch(error) {
      console.error('Error Logging into LMS:', error);
    }
  }
  return (
    <div className="h-screen w-screen flex justify-center flex-col gap-8 items-center bg-white">
      <img className="h-10" src={Moodlelogo}/>
      <input className="rounded-xl border-2 border-gray-200 bg-white p-3" type="text" placeholder="Username" onChange={(e) => setUsername(e.target.value)} />
      <input className="rounded-xl border-2 border-gray-200 bg-white p-3" type="password" placeholder="Password" onChange={(e) => setPassword(e.target.value)} />
      <button className="bg-black text-white px-16 py-4 rounded-2xl" onClick={handleSubmit}>Submit</button>
    </div>
  );
}