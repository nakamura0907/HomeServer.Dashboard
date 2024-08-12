import { useState } from "react";

type DashboardItem = {
  title: string;
  url: string;
};

type FormValues = Partial<DashboardItem>;

type State = {
  list: DashboardItem[];
  formValues: FormValues;
};

const initialState: State = {
  list: [],
  formValues: {},
};

export default function Home() {
  const [list, setList] = useState(initialState.list);
  const [formValues, setFormValues] = useState(initialState.formValues);

  return (
    <main>
      <div>{/* form */}</div>
      <div>
        <ul>
          {list.map((item, index) => {
            return (
              <li key={index}>
                <a href={item.url} target="_blank" rel="noopener noreferrer">
                  {item.title}
                </a>
              </li>
            );
          })}
        </ul>
      </div>
    </main>
  );
}
