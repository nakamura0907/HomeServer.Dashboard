"use client";
import { FormEvent, useState } from "react";

type DashboardItem = {
  title: string;
  url: string;
};

type FormValues = DashboardItem;

type State = {
  list: DashboardItem[];
  formValues: FormValues;
};

const initialState: State = {
  list: [],
  formValues: {
    title: "",
    url: "",
  },
};

export default function Home() {
  const [list, setList] = useState(initialState.list);
  const [formValues, setFormValues] = useState(initialState.formValues);

  const handleChangeFormValue = <T extends keyof FormValues>(
    key: T,
    value: FormValues[T]
  ) => {
    setFormValues({
      ...formValues,
      [key]: value,
    });
  };

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    console.log(formValues);

    // バリデーション
    const { title, url } = formValues;
    if (!title || !url) return;

    // リストの更新
    setList([{ title, url }, ...list]);

    // 終了処理
    setFormValues(initialState.formValues);
  };

  return (
    <main>
      <div>
        <form onSubmit={handleSubmit}>
          <div>
            <label htmlFor="title">TITLE</label>
            <input
              type="text"
              name="title"
              value={formValues.title}
              onChange={(e) => handleChangeFormValue("title", e.target.value)}
            />
          </div>
          <div>
            <label htmlFor="url">URL</label>
            <input
              type="text"
              name="url"
              value={formValues.url}
              onChange={(e) => handleChangeFormValue("url", e.target.value)}
            />
          </div>
          <div>
            <button type="submit">SUBMIT</button>
          </div>
        </form>
      </div>
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
