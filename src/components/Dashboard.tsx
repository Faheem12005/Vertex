import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import Navbar from "./Navbar.tsx";
import DashboardSection from "./DashboardSection.tsx";
import Description from "./Description.tsx";

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
    return response[0].data.events.map((event: any) => ({
      name: event.name,
      description: event.description,
      instanceId: event.instance,
      courseName: event.course.fullname, 
      dueDate: event.formattedtime, 
    }));
  } catch (error) {
    console.error("Failed to fetch assignments:", error);
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
