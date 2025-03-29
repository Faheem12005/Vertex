import "./App.css";
import Login from "./components/Login";
import React, { createContext, useState } from "react";
import Dashboard from "./components/Dashboard";
import { Toaster } from "react-hot-toast";

type AuthContextType = {
  loggedIn: boolean;
  setLoggedIn: React.Dispatch<React.SetStateAction<boolean>>;
};

export const AuthContext = createContext<AuthContextType>({
  loggedIn: false,
  setLoggedIn: () => {},
});

function App() {
  const [loggedIn, setLoggedIn] = useState<boolean>(false);

  return (
    <AuthContext.Provider value={{ loggedIn, setLoggedIn }}>
      <Toaster />
      { loggedIn ? 
      <>
      <Dashboard/>
      </> 
      : 
      <>
      <Login/>
      </>
      }
    </AuthContext.Provider>
  );
}

export default App;
