import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { useContext, useEffect, useRef } from "react";
import { AuthContext } from "../App";
import { ErrorKind } from "../models/errors.ts";
import toast from 'react-hot-toast';

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
        } catch (e) {
            console.error(e);
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
      let response: string = await invoke('login_lms', { payload: { username: username, password: password } });
      console.log(response);
      localStorage.setItem("username", response);
      setLoggedIn(true);
    } catch (e) {
        console.log("error occurred while logging in", e);
        if (typeof e === "object" && e !== null && "kind" in e && "message" in e) {
            const error = e as ErrorKind; // Type assertion
            switch (error.kind) {
                case "authError":
                    toast.error("Incorrect Credentials", {
                        position: "bottom-center",
                        id: "auth"
                    });
                    break;
                case "networkError":
                    toast.error("failed to Send request, try again later", {
                        position: "bottom-center",
                        id: "network"
                    });
                    break;
            }
        }
    } finally {
      setLoading(false);
    }
  }
  return (
      <>
          {loadingLogin ? (
                  <div className="fixed inset-0 flex flex-col gap-20 items-center justify-center bg-white">
                      <img
                          src="/icons/Spinner.svg"
                          className="h-16 w-16 animate-spin opacity-80"
                          alt="Loading..."
                      />
                      <p className="font-primary text-gray-400">Loading.....</p>
                  </div>
              )
              :
              <div className="h-screen w-screen flex justify-center flex-col gap-8 items-center bg-white">
                  <img className="h-10" src="/icons/Moodlelogo.svg" alt={"moodle_logo"}/>
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
                      {loading && <img src="/icons/Spinner.svg" className="h-10 w-10 animate-spin" alt="spinner" />}
                  </div>
              </div>
          }
      </>
  );
}