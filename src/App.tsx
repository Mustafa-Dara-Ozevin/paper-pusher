import { invoke } from "@tauri-apps/api/tauri";
import React, { Children, useState } from "react";
import {
  BankOutlined,
  CarOutlined,
  DashboardOutlined,
  AlertOutlined,
  AimOutlined,
  ExperimentOutlined,
  HomeOutlined,
} from "@ant-design/icons";
import type { MenuProps } from "antd";
import { Menu } from "antd";
import { register } from "@tauri-apps/api/globalShortcut";
import { writeText } from "@tauri-apps/api/clipboard";
import { enable } from "tauri-plugin-autostart-api";

register("Alt+L", async () => {
  await invoke("show_window");
});

const items: MenuProps["items"] = [
  {
    label: "Traffic Stops",
    key: "SubMenu",
    icon: <DashboardOutlined />,
    children: [
      {
        label: "Speeding Ticket",
        key: "speedingTicket",
      },
      {
        label: "Failure to obey traffic control devices",
        key: "controlDevices",
      },
      {
        label: "Suspended License",
        key: "suspendedLicense",
      },
      {
        label: "Negligent Driving",
        key: "negligentDriving",
      },
    ],
  },
  {
    label: "Robbery of a financial institue",
    key: "robbery",
    icon: <BankOutlined />,
  },
  {
    label: "Grand Theft Auto",
    key: "boost",
    icon: <CarOutlined />,
  },
  {
    label: "Meth/Heroin Run",
    key: "methRun",
    icon: <AlertOutlined />,
  },
  {
    label: "Sale of Drugs",
    key: "saleOfDrugs",
    icon: <ExperimentOutlined />,
  },
  {
    label: "Gang Related Shooting",
    key: "GRS",
    icon: <AimOutlined />,
  },
  {
    label: "House robbery",
    key: "houseRobbery",
    icon: <HomeOutlined />,
  },
];

const getName = () => {
  if (
    localStorage.getItem("officerName") === null ||
    localStorage.getItem("officerName") === ""
  ) {
    const name: any = prompt("Enter your badge number and name");
    localStorage.setItem("officerName", name);
  }
};

const App: React.FC = () => {
  async () => {
    await enable();
  };
  getName();

  const onClick: MenuProps["onClick"] = async (e) => {
    getName();
    let suspect: any = prompt("Enter suspect name: ");
    let name: any = localStorage.getItem("officerName");
    if (name === null || name === "" || suspect === null || suspect === "") {
      return;
    }
    await invoke("hide_window");
    let text: string = await invoke("template", { key: e.key });

    text =
      name +
      "'s Statement: \n" +
      text.replaceAll("OFFICER", name).replaceAll("SUSPECT", suspect);

    await writeText(text);
  };

  return (
    <Menu
      id="menu"
      onClick={onClick}
      mode="vertical"
      theme="dark"
      items={items}
    />
  );
};

export default App;
