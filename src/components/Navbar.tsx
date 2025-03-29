import { invoke } from "@tauri-apps/api/core";
import { useContext } from "react";
import { AuthContext } from "../App";
import toast from "react-hot-toast";
export default function Navbar () {
    const {setLoggedIn} = useContext(AuthContext);

    const handleLogout = async () => {
        try {
            console.log("Logging out...");
            await invoke("logout_lms");
            localStorage.removeItem("username");
            setLoggedIn(false);
        } catch (error) {
            toast.error("Error logging out", {
                position: "bottom-right",
                duration: 2000,
            });
        }
    };

    return (
        <div className="w-full px-5 py-5 flex items-center justify-between">
            <p>@Vertex</p>
            <p className="font-medium font-primary">{localStorage.getItem("username")}</p>
            <button className="hover:underline" onClick={handleLogout}>Logout.</button>
        </div>
    );
}