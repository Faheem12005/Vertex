interface Description {
    description?: string;
    title?: string;
    id?: string;
    opened_date?: string;
    due_date?: string;
    file_url?: Array<Array<string>>;
}

interface DescriptionProps {
    description: Description | undefined;
}


export default function Description (description: DescriptionProps) {

    return (
        <div className="p-5 flex-col col-span-2 h-full rounded-3xl bg-gray-100 overflow-y-hidden font-primary">
            <p className="text-3xl mb-5">{description.description?.title}</p>
            <p className="text-xl mb-3">{description.description?.description}</p>
            {description.description?.opened_date && (
                <p><span className="font-bold">Opened on:</span> {description.description.opened_date}</p>
            )}

            {description.description?.due_date && (
                <p className="mb-2"><span className="font-bold">Due by:</span> {description.description.due_date}</p>
            )}
            {description.description?.file_url?.map(([url, text]) => (
                <a className="block font-light text-blue-500 hover:underline" key={url} href={url} target="_blank" rel="noreferrer">{text}</a>
            ))}
        </div>
    );
}