export interface Template {
  link: string;
  type: string;
  title: string;
};
const one: Template = {
  link: "mailto:email@example.com",
  type: "Email",
  title: "silen.locatelli@gmx.ch",
};
const two: Template = {
link: "www.linkedin.com/in/silen-locatelli",
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
