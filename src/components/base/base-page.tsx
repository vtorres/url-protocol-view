import { Typography } from "@mui/material";
import React from "react";

interface Props {
  title?: React.ReactNode;
  header?: React.ReactNode;
  contentStyle?: React.CSSProperties;
  children: React.ReactNode;
}

const BasePage: React.FC<Props> = (props) => {
  const { title, header, contentStyle, children } = props;

  return (
    <div className="base-page" data-windrag>
      <header data-windrag style={{ userSelect: "none" }}>
        <Typography variant="h5" component="h1" data-windrag>
          {title}
        </Typography>

        {header}
      </header>

      <section>
        <div className="base-content" style={contentStyle} data-windrag>
          {children}
        </div>
      </section>
    </div>
  );
};

export default BasePage;
