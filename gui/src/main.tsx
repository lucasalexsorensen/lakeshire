import React from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  Outlet,
  RouterProvider,
} from "react-router-dom";
import "./style.css";
import Index from "./pages/Index";
import Editor from "./pages/Editor";
import Settings from "./pages/Settings";
import Navigation from "./components/Navigation";
import { useLocation } from 'react-router-dom';


function Wrapper () {
  const { pathname } = useLocation();
  return (
    <div className="bg-slate-800 text-gray-100 h-screen w-screen font-mono overflow-hidden">
      {pathname !== "/" && <Navigation />}
      <Outlet />
    </div>
  )
}

const router = createBrowserRouter([
  {
    path: "/",
    element: <Wrapper />,
    children: [
      {
        path: "/",
        element: <Index />
      },
      {
        path: "/editor",
        element: <Editor />
      },
      {
        path: "/settings",
        element: <Settings />
      }
    ]
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
