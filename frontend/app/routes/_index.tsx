import type { MetaFunction } from "@remix-run/node";

export const meta: MetaFunction = () => {
  return [
    { title: "New Remix App" },
    { name: "description", content: "Welcome to Remix!" },
  ];
};

export default function Index() {
  return (
    <div className="card bg-base-300 animate-fade-up">
      <div className="card-body">Programmers Playground</div>
    </div>
  );
}
