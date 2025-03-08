import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { useContext } from "react";
import { AuthContext } from "../App";
import Moodlelogo from "../assets/icons/Moodlelogo.svg"

export default function Dashboard () {

  return (
    <div className="h-screen w-screen flex justify-center flex-col gap-8 items-center bg-white">
        <p>Welcome! {localStorage.getItem("username")}</p>
    </div>
  );
}