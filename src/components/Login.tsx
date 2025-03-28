import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { useContext, useEffect, useRef } from "react";
import { AuthContext } from "../App";
import Moodlelogo from "../../public/icons/Moodlelogo.svg"
import Spinner from "../../public/icons/Spinner.svg"
export default function Login () {

  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const { setLoggedIn } = useContext(AuthContext);
  const [loadingLogin, setLoadingLogin] = useState(true);

    const hasRun = useRef(false);

    const checkIfLoggedIn = async () => {
        try {
            let response = await invoke("auto_login_lms");
            console.log(response);
            setLoggedIn(true);
        } catch (error) {
            console.error("Error autologging in ", error);
        } finally {
            setLoadingLogin(false);
        }
    };

    useEffect(() => {
        if (!hasRun.current) {
            hasRun.current = true;
            checkIfLoggedIn();
        }
    }, []);

  const handleSubmit = async() => {
    setLoading(true);
    try {
        console.log("handle submit called");
      let response: string = await invoke('login_lms', { payload: JSON.stringify({ username: username, password: password }) });
      console.log(response);
      localStorage.setItem("username", response);
      setLoggedIn(true);
    } catch(error) {
      console.error('Error Logging into LMS:', error);
    } finally {
      setLoading(false);
    }
  }
  return (
      <>
          {loadingLogin ? <img src={Spinner} className="h-10 w-10 animate-spin" alt="spinner" />
              :
              <div className="h-screen w-screen flex justify-center flex-col gap-8 items-center bg-white">
                  <img className="h-10" src={Moodlelogo} alt={"moodle_logo"}/>
                  <input className="rounded-xl border-2 border-gray-200 bg-white p-3" type="text" placeholder="Username"
                         onChange={(e) => setUsername(e.target.value)}/>
                  <input className="rounded-xl border-2 border-gray-200 bg-white p-3" type="password" placeholder="Password"
                         onChange={(e) => setPassword(e.target.value)}/>
                  <button
                      disabled={loading}
                      className="bg-primary-500 text-white px-16 py-4 rounded-2xl font-bold hover:bg-primary-600 focus:outline-2 focus:outline-offset-2 focus:outline-primary-500 active:bg-primary-700"
                      onClick={handleSubmit}>Submit
                  </button>
                  <div className="h-3 mt-2 flex justify-center">
                      {loading && <img src={Spinner} className="h-10 w-10 animate-spin" alt="spinner" />}
                  </div>
              </div>
          }
      </>
  );
}