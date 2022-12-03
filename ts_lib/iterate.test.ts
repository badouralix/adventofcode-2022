import { assertEquals } from "https://deno.land/std@0.167.0/testing/asserts.ts";

import { iterate } from "@lib/iterate.ts";

Deno.test("can match with 1 char long", () => {
  const input = "a\nb\nc\nd";
  const elements: string[] = [];
  for (const element of iterate(input, "\n")) {
    elements.push(element);
  }
  assertEquals(elements, ["a", "b", "c", "d"]);
});

Deno.test("can match with 2 chars long", () => {
  const input = "a\nb\n\nc\nd";
  const elements: string[] = [];
  for (const element of iterate(input, "\n\n")) {
    elements.push(element);
  }
  assertEquals(elements, ["a\nb", "c\nd"]);
});

Deno.test("work without match", () => {
  const input = "a\nb\nc\nd";
  const elements: string[] = [];
  for (const element of iterate(input, "\t")) {
    elements.push(element);
  }
  assertEquals(elements, ["a\nb\nc\nd"]);
});

Deno.test("work with empty strings", () => {
  const input = "";
  const elements: string[] = [];
  for (const element of iterate(input, "\n")) {
    elements.push(element);
  }
  assertEquals(elements, [""]);
});
