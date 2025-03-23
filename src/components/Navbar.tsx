import { invoke } from "@tauri-apps/api/core";
import { useContext } from "react";
import { AuthContext } from "../App";
export default function Navbar () {
    const {setLoggedIn} = useContext(AuthContext);

    const handleLogout = async() => {
        try {
            await invoke("logout_lms");
            localStorage.removeItem("username");
            setLoggedIn(false);
        } catch(error) {
            console.error("Failed to logout:", error);
        }
    }
    return (
        <div className="w-full px-5 py-5 flex items-center justify-between">
            <p>@Vertex</p>
            <p className="font-medium font-primary">{localStorage.getItem("username")}</p>
            <button className="hover:underline" onClick={handleLogout}>Logout.</button>
        </div>
    );
}