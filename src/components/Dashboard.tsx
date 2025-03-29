import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import Navbar from "./Navbar.tsx";
import DashboardSection from "./DashboardSection.tsx";
import Description from "./Description.tsx";
import { ErrorKind } from "../models/errors.ts";
import toast from "react-hot-toast";

export interface Assignments {
  name: string,
  description: string,
  instanceId: string,
  courseName: string,
  dueDate: string,
}

const fetchAssignments = async () => {
  try {
    let response = JSON.parse(await invoke("fetch_assignments"));
    console.log(response)
    localStorage.setItem("assignments", JSON.stringify(response));
    toast.success("fetched assignments successfully.", {
      position: "bottom-right",
      id: 'assignments',
    });
    return response[0].data.events.map((event: any) => ({
      name: event.name,
      description: event.description,
      instanceId: event.instance,
      courseName: event.course.fullname, 
      dueDate: event.formattedtime, 
    }));

  } catch (e) {
    console.error("Failed to fetch assignments:", e);
    if (typeof e === "object" && e !== null && "kind" in e && "message" in e) {
      const error = e as ErrorKind; // Type assertion
      switch (error.kind) {
        case "networkError":
          toast.error("failed to fetch assignments, try again later", {
            position: "bottom-right",
            id: 'assignments',
          });
          break;
      }
    }
    return [];
  }
};

export default function Dashboard() {
  const [assignments, setAssignments] = useState<Assignments[]>([]);
  const [description, setDescription] = useState();
  useEffect(() => {
    fetchAssignments().then(setAssignments);
  }, []);

  // @ts-ignore
  return (
        <div className="flex flex-col min-h-screen">
          <Navbar/>
          <button onClick={() => fetchAssignments().then(setAssignments)} className="font-primary hover:cursor-pointer">CLick to refresh</button>
          <div className="grid grid-cols-3 flex-grow p-4 gap-5">
            <DashboardSection
                heading="LMS"
                cards={assignments}
                setDescription={setDescription}
            />
            <Description
                description={description}
            />
          </div>
        </div>
  );
}
