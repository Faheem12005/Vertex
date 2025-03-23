import { Assignments } from "./Dashboard.tsx";
import Card from "./Card.tsx";

export default function DashboardSection({cards, heading, setDescription} : { cards: Assignments[], heading: string , setDescription: Function}) {
    return (
        <div className="p-5 flex-1 h-full rounded-3xl bg-gray-100">
            <div className="flex items-center justify-center bg-secondary-500 h-20 rounded-3xl p-4 mt-5 flex-col font-primary">
                <p className="font-bold text-3xl tracking-widest">{heading}</p>
            </div>
            {cards.map(card => (
                <Card heading={card.courseName}
                      duedate={card.dueDate}
                      key={card.instanceId}
                      id={card.instanceId}
                      setDescription={setDescription}
                />
            ))}
        </div>
    )
}