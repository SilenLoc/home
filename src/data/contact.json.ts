export interface Template {
  link: string;
  type: string;
  title: string;
};
const one: Template = {
  link: "mailto:silen.locatelli@gmx.ch",
  type: "Email",
  title: "silen.locatelli@gmx.ch",
};
const two: Template = {
link: "https://www.linkedin.com/in/silen-locatelli",
    type: "Linkedin",
  title: "Silen Locatelli",
};
const three: Template = {
  link: "https://github.com/SilenLoc",
    type: "Gihutb",
    title: "SilenLoc",
  };

export const bytype = {
  one,
  two,
  three
};
export const contact = Object.values(bytype);
