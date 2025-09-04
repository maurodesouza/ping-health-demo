-- CreateTable
CREATE TABLE "public"."pings" (
    "id" TEXT NOT NULL,
    "service" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "pings_pkey" PRIMARY KEY ("id")
);
